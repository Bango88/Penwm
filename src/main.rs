#[macro_use]
extern crate penrose;

use std::{thread, time};

use penrose::{
    core::{
        bindings::KeyEventHandler,
        config::Config,
        helpers::index_selectors,
        data_types::{Region, WinType},
        hooks::Hook,
        xconnection::Atom,
        manager::WindowManager,
    },
    draw::{bar::dwm_bar, Draw, DrawContext, TextStyle},
    logging_error_handler,
    xcb::{new_xcb_backed_window_manager, XcbDraw},
    Backward, Forward, Less, More, Selector,
    Result,
};

use simplelog::{LevelFilter, SimpleLogger};


// Replace these with your preferred terminal and program launcher
const TERMINAL: &str = "alacritty";
const LAUNCHER: &str = "dmenu_run";


const HEIGHT: usize = 18;

const PROFONT: &str = "ProFont For Powerline";
const FIRA: &str = "Fira Code";
const SERIF: &str = "Serif";

const BLACK: u32 = 0x282828ff;
const GREY: u32 = 0x3c3836ff;
const WHITE: u32 = 0xebdbb2ff;
const PURPLE: u32 = 0xb16286ff;
const BLUE: u32 = 0x458588ff;
const RED: u32 = 0xcc241dff;


fn main() -> penrose::Result<()> {
    // Initialise the logger (use LevelFilter::Debug to enable debug logging)
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    //simple_draw()?;
    let config = Config::default();
    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-d" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);

        // Exit Penrose (important to remember this one!)
        "M-A-C-Escape" => run_internal!(exit);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-S-q" => run_internal!(kill_client);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-A-period" => run_internal!(cycle_workspace, Forward);
        "M-A-comma" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    //bar_draw()?;

    let workspaces = vec!["1", "2", "3", "4", "5", "6"];
    let style = TextStyle {
        font: PROFONT.to_string(),
        point_size: 11,
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2.0, 2.0),
    };
    let highlight = BLUE;
    let empty_ws = GREY;
    let mut bar = dwm_bar(
        XcbDraw::new()?,
        HEIGHT,
        &style,
        highlight,
        empty_ws,
        workspaces,
    )?;

    //let mut wm = new_xcb_backed_window_manager(Config::default(), vec![], logging_error_handler())?;
    bar.startup(&mut wm)?;

    wm.grab_keys_and_run(key_bindings, map!{});
    Ok(())
}


/*
fn bar_draw() -> Result<()> {
    let workspaces = vec!["1", "2", "3", "4", "5", "6"];
    let style = TextStyle {
        font: PROFONT.to_string(),
        point_size: 11,
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2.0, 2.0),
    };
    let highlight = BLUE;
    let empty_ws = GREY;
    let mut bar = dwm_bar(
        XcbDraw::new()?,
        HEIGHT,
        &style,
        highlight,
        empty_ws,
        workspaces,
    )?;

    //let mut wm = new_xcb_backed_window_manager(Config::default(), vec![], logging_error_handler())?;
    bar.startup(&mut wm)?; // ensure widgets are initialised correctly
    /*
    thread::sleep(time::Duration::from_millis(1000));
    for focused in 1..6 {x
        bar.workspace_change(&mut wm, focused - 1, focused)?;
        bar.event_handled(&mut wm)?;
        thread::sleep(time::Duration::from_millis(1000));
    }*/

    //thread::sleep(time::Duration::from_millis(10000));
    Ok(())
}*/

fn simple_draw() -> Result<()> {
    let mut drw = XcbDraw::new()?;
    let (_, _, w, _) = drw.screen_sizes()?[0].values();
    let id = drw.new_window(
        WinType::InputOutput(Atom::NetWindowTypeNormal),
        Region::new(0, 0, w, HEIGHT as u32),
        false,
    )?;
    drw.register_font(PROFONT);
    drw.register_font(SERIF);
    drw.register_font(FIRA);

    let mut ctx = drw.context_for(id)?;

    ctx.color(&WHITE.into());
    ctx.rectangle(0.0, 0.0, w as f64, HEIGHT as f64);
    ctx.translate(1.0, 1.0);

    ctx.color(&BLACK.into());
    ctx.font(PROFONT, 12)?;
    let (offset, _) = ctx.text("this is a simple test", 0.0, (0.0, 8.0))?;

    ctx.color(&RED.into());
    ctx.font(SERIF, 10)?;
    ctx.translate((offset + 5.0) as f64, 0.0);
    let (offset, _) = ctx.text("BORK BORK!", 0.0, (0.0, 0.0))?;

    ctx.color(&PURPLE.into());
    ctx.font(FIRA, 10)?;
    ctx.translate((offset + 5.0) as f64, 0.0);
    ctx.text("Look at all the colors!", 0.0, (0.0, 0.0))?;

    drw.flush(id)?;
    thread::sleep(time::Duration::from_millis(5000));
    Ok(())

}