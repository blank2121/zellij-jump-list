use ansi_term::{Colour::Fixed, Style};
use std::collections::{BTreeMap, HashSet};
use zellij_tile::prelude::*;
use std::cmp::{min, max};

#[derive(Debug, Default, Clone, PartialEq)]
struct CustomPane {
    id: u32,
    name: String,
    is_plugin: bool,
}

impl std::fmt::Display for CustomPane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default)]
struct State {
    userspace_configuration: BTreeMap<String, String>,
    current_focus: CustomPane,
    current_mode: InputMode,
    previous_mode: InputMode,
    jump_list: Vec<CustomPane>,
    select_focus: i32,
    debug: String,
}

impl State {
    fn current_pane(&self, zellij_instance: &PaneManifest) -> Option<CustomPane> {
        let tabs: Vec<&usize> = zellij_instance.panes.keys().collect::<Vec<&usize>>();

        for t in tabs.iter() {
            let panes_in_tab: &Vec<PaneInfo> = zellij_instance.panes.get(t).unwrap();

            for p in panes_in_tab.iter() {
                if p.is_focused {
                    return Some(CustomPane {
                        id: p.id,
                        name: p.title.clone(),
                        is_plugin: p.is_plugin,
                    });
                }
            }
        }
        None
    }

    // supposed to remove panes that no longer exist
    // but there is not way to do that yet with 0.38.0 zellij
    // --------------- TODO ---------------
    fn purge(&mut self, zellij_instance: PaneManifest) {
        let mut existing_ids: HashSet<PaneInfo> = HashSet::new();

        // getting list of all pane ids that exist
        let tabs: Vec<&usize> = zellij_instance.panes.keys().collect::<Vec<&usize>>();

        for t in tabs.iter() {
            let panes_in_tab: &Vec<PaneInfo> = zellij_instance.panes.get(t).unwrap();

            for p in panes_in_tab.iter() {
                existing_ids.insert(p.clone());
            }
        }

        // debug for detecting if pane is deleted
        self.debug = format!("ids that exist: {:?} | status: {:?}", 
                             existing_ids.iter().map(|x| x.id).collect::<Vec<u32>>(),
                             existing_ids.iter().map(|x| x.exit_status).collect::<Vec<Option<i32>>>());

        self.jump_list.retain(|x| {
            existing_ids.iter()
                .map(|a| a.id)
                .collect::<Vec<u32>>()
                .contains(&x.id)
        });
    }

    fn is_previous_jump(&self) -> bool {
        if self.jump_list.get(0).is_none() {
            false
        } else {
            if self.jump_list.get(0).unwrap() == &self.current_focus {
                return true;
            }
            false
        }
    }
}
register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.userspace_configuration = configuration;

        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::RunCommands,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::ModeUpdate, EventType::Key, EventType::PaneUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;

        match event {

            // when going to normal mode, it is added to the list
            Event::ModeUpdate(mode) => {
                self.current_mode = mode.mode;
        
                if mode.mode != InputMode::Normal || mode.mode == self.previous_mode {
                    self.previous_mode = mode.mode;
                    return true;
                }
                if self.current_focus.is_plugin {
                    return true;
                }
                if self.is_previous_jump() {
                    return true;
                }

                // updating self.jump_list & self.previous_mode
                let mut temp: Vec<CustomPane> = vec![self.current_focus.clone()];
                temp.extend(self.jump_list.clone());

                if temp.len() > 10 {
                    temp = temp[..10].to_vec();
                }

                self.jump_list = temp;
                self.previous_mode = mode.mode;
                should_render = true;
            }

            Event::Key(key) => {
                if key == Key::Esc {
                    hide_self();
                }
                if key == Key::Char('\n') {
                    // select and hide in jump list

                    let index_for_pane = (self.select_focus - 1) as usize;
                    let to_focus: u32 = self.jump_list[index_for_pane].id;
                    focus_terminal_pane(to_focus, true);
                    hide_self();
                }
                if key == Key::Char('k') || key == Key::Up {
                    self.select_focus = max(1, self.select_focus-1);
                }
                if key == Key::Char('j') || key == Key::Down {
                    let ten_or_current = min(self.jump_list.len() as i32, 10);
                    self.select_focus = min(self.select_focus+1, ten_or_current); 
                }

                should_render = true;
            }
            Event::PaneUpdate(pane_info) => {
                self.current_focus = self.current_pane(&pane_info).unwrap();
                // guard clauses
                if self.current_focus.is_plugin {return true;}
                if self.is_previous_jump() {return true;}
                if self.current_mode != InputMode::Normal {return true;}

                // temp will be the updated jump list
                let mut temp: Vec<CustomPane> = vec![self.current_focus.clone()];
                temp.extend(self.jump_list.clone());


                self.select_focus = 1;

                if temp.len() >= 2 {self.select_focus = 2;}

                if temp.len() > 10 {
                    temp = temp[..10].to_vec();
                    self.select_focus = 2;
                }

                self.jump_list = temp;
                self.purge(pane_info);
                should_render = true;
            }
            _ => (),
        };

        should_render
    }
    fn render(&mut self, _rows: usize, _cols: usize) {
        // tui title
        println!("{}", Style::new().fg(Fixed(GREEN)).bold().paint("Jump List\n"));

        // tui listing out jump list
        for (idx, x) in self.jump_list.iter().enumerate() {
            if (idx as i32) != self.select_focus-1 {
                println!("{}. {}", idx+1, x)
            } else {
                println!("{}. {}", idx+1, color_bold(RED, &x.to_string()))
            }
        }
    }
}

pub const CYAN: u8 = 51;
pub const GRAY_LIGHT: u8 = 238;
pub const GRAY_DARK: u8 = 245;
pub const WHITE: u8 = 15;
pub const BLACK: u8 = 16;
pub const RED: u8 = 124;
pub const GREEN: u8 = 154;
pub const ORANGE: u8 = 166;

fn color_bold(color: u8, text: &str) -> String {
    format!("{}", Style::new().fg(Fixed(color)).bold().paint(text))
}
