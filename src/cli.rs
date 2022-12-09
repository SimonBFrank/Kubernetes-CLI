use crate::models::configuration::config_dict::Config;
use crate::models::configuration::env_config::EnvConfig;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use std::io::{stdout, Write};

pub fn print_selector(items: &Vec<String>, idx: usize, offset: u32) {
    let mut s: String = String::new();
    for (i, env) in items.iter().enumerate() {
        if i == idx {
            s.push_str(&format!("> {env}\n"));
        } else {
            s.push_str(&format!("  {env}\n"));
        }
    }
    let mut stdout = stdout();
    execute!(
        stdout,
        cursor::MoveTo(0, offset as u16),
        Clear(ClearType::FromCursorDown),
        Print(s)
    )
    .unwrap();
    return ();
}

fn pick_color(color_cfg: EnvConfig, offset: u32) {
    let selected_env = color_cfg.env.clone();
    let mut colors = color_cfg.get_keys();
    colors.push(String::from("BACK"));
    let colors = colors;
    let n_colors: usize = colors.len();

    let mut line_choice = 0;
    let mut stdout = stdout();

    execute!(
        stdout,
        Clear(ClearType::FromCursorDown),
        cursor::MoveTo(0, offset as u16)
    )
    .unwrap();

    loop {
        print_selector(&colors, line_choice, offset);

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: _,
            }) => {
                if line_choice > 0 {
                    line_choice = line_choice - 1;
                }
                print_selector(&colors, line_choice, offset);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: _,
            }) => {
                if line_choice < n_colors - 1 {
                    line_choice = line_choice + 1;
                }
                print_selector(&colors, line_choice, offset);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: _,
            }) => {
                let selected_color = colors[line_choice.clone()].clone();

                if (n_colors - 1) == line_choice {
                    return ();
                } else {
                    let s: String = format!(
                        "Selected environment: {selected_env}, selected color: {selected_color} \n"
                    );
                    execute!(
                        stdout,
                        cursor::MoveTo(0, offset as u16),
                        Clear(ClearType::FromCursorDown),
                        Print(s)
                    )
                    .unwrap();
                    std::process::exit(0);
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => std::process::exit(0),
            _ => (),
        }
    }
}

pub fn pick_env(cfg: Config, offset: u32) {
    let mut envs = cfg.get_keys();
    envs.push(String::from("EXIT"));
    let envs = envs;
    let n_envs = envs.len();

    let mut line_choice = 0;
    let mut stdout = stdout();

    execute!(
        stdout,
        cursor::MoveTo(0, offset as u16),
        Clear(ClearType::FromCursorDown)
    )
    .unwrap();

    loop {
        print_selector(&envs, line_choice, offset);

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: _,
            }) => {
                if line_choice > 0 {
                    line_choice = line_choice - 1;
                }
                print_selector(&envs, line_choice, offset);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: _,
            }) => {
                if line_choice < n_envs - 1 {
                    line_choice = line_choice + 1;
                }
                print_selector(&envs, line_choice, offset);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: _,
            }) => {
                let selected_env = envs[line_choice.clone()].clone();

                if (n_envs - 1) == line_choice {
                    return ();
                } else {
                    let env_cfg = cfg[selected_env].clone();
                    pick_color(env_cfg, offset);
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => std::process::exit(0),
            _ => (),
        }
    }
}

pub fn start(cfg: Config) {
    let mut stdout = stdout();
    let welcome_message = concat!(
        "  Welcome to Kubernetes CLI!  \n",
        "  Use arrow keys and enter to navigate, control-c to exit.\n",
        "-----------------------------------------------------------"
    );
    let offset: u32 = welcome_message.split('\n').count() as u32;
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print(welcome_message)
    )
    .unwrap();
    pick_env(cfg, offset);
}
