//! CRIMSON-REDLINE Terminal Hacking Simulator
//! Main entry point and application orchestration

use crimson_redline::*;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
    execute,
    cursor,
};
use std::io::{self, Write};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the application
    let result = run_application().await;
    
    // Ensure terminal is restored on exit
    let _ = terminal::disable_raw_mode();
    let _ = execute!(io::stdout(), cursor::Show);
    
    if let Err(e) = result {
        eprintln!("\n[ERROR] Application crashed: {}", e);
        std::process::exit(1);
    }
    
    println!("\n[SYSTEM] Connection terminated.\n");
    Ok(())
}

/// Main application loop
async fn run_application() -> Result<()> {
    // Initialize UI
    let mut ui = ui::RedlineUI::new()?;
    
    // Show intro animation on a separate screen
    execute!(
        io::stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide
    )?;
    
    // Show intro animation
    ui.initialize().await?;
    
    // Clear and exit alternate screen after intro
    execute!(
        io::stdout(),
        terminal::LeaveAlternateScreen,
        cursor::Show
    )?;
    
    // Small delay to ensure clean transition
    sleep(Duration::from_millis(100)).await;
    
    // Clear main screen
    utils::clear_screen()?;
    
    // Load configuration
    let _config = utils::Config::load()?;
    
    // Initialize authentication system
    let mut auth_system = auth::AuthSystem::new()?;
    
    // Main application loop
    loop {
        // Show entry menu (CREATE NEW USER / LOGIN / EXIT)
        let menu_action = show_entry_menu(&mut auth_system, ui.color_scheme()).await?;
        
        match menu_action {
            ui::menu::MenuAction::Exit => {
                show_exit_sequence(ui.color_scheme()).await?;
                break;
            }
            ui::menu::MenuAction::Continue => {
                // User successfully logged in, enter main terminal
                if let Some(user) = auth_system.current_user() {
                    run_terminal_session(user.clone(), &mut auth_system, &mut ui).await?;
                }
            }
            _ => continue,
        }
    }
    
    // Cleanup
    ui.cleanup()?;
    Ok(())
}

/// Show entry menu and handle authentication (FIXED FOR NO FLICKERING)
async fn show_entry_menu(
    auth_system: &mut auth::AuthSystem,
    color_scheme: &ui::ColorScheme,
) -> Result<ui::menu::MenuAction> {
    let mut menu = ui::menu::MainMenu::entry_menu();
    let mut needs_redraw = true;
    
    loop {
        // Only redraw when needed
        if needs_redraw {
            terminal::enable_raw_mode()?;
            menu.display(color_scheme).await?;
            needs_redraw = false;
        }
        
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let action = menu.handle_input(key);
                
                match action {
                    ui::menu::MenuAction::Continue => {
                        needs_redraw = true;  // Redraw for navigation
                        continue;
                    }
                    ui::menu::MenuAction::CreateNewUser => {
                        terminal::disable_raw_mode()?;
                        let success = auth::register::run_registration(auth_system, color_scheme).await?;
                        if success {
                            utils::clear_screen()?;
                            color_scheme.print_success("\n  [✓] Registration successful! Please login with your new credentials.\n\n")?;
                            sleep(Duration::from_secs(2)).await;
                        }
                        utils::clear_screen()?;  // Clear screen before showing menu again
                        needs_redraw = true;  // Force menu redraw
                        continue;  // Continue the loop - DON'T return, stay in menu
                    }
                    ui::menu::MenuAction::Login => {
                        terminal::disable_raw_mode()?;
                        let success = auth::login::run_login(auth_system, color_scheme).await?;
                        if success {
                            return Ok(ui::menu::MenuAction::Continue);  // Only return on successful login
                        }
                        utils::clear_screen()?;  // Clear screen before showing menu again
                        needs_redraw = true;  // Force menu redraw
                        continue;  // Continue the loop if login failed
                    }
                    ui::menu::MenuAction::Exit => {
                        terminal::disable_raw_mode()?;
                        return Ok(ui::menu::MenuAction::Exit);  // Only return on exit
                    }
                    _ => {
                        needs_redraw = true;
                        continue;
                    }
                }
            }
        }
    }
}

/// Run the main terminal session after login
async fn run_terminal_session(
    user: auth::User,
    auth_system: &mut auth::AuthSystem,
    ui: &mut ui::RedlineUI,
) -> Result<()> {
    // Clear screen and show welcome
    utils::clear_screen()?;
    show_welcome_message(&user, ui.color_scheme()).await?;
    
    // Initialize command handler with user's game state
    let mut command_handler = commands::CommandHandler::new(&user);
    
    // Initialize event manager
    let mut event_manager = game::events::EventManager::new();
    
    // Command prompt (removed mut as it's not needed)
    let prompt = ui::menu::CommandPrompt::new(user.username.clone());
    
    // Main terminal loop
    loop {
        // Check for random events
        if let Some(event) = event_manager.generate_event(
            command_handler.game_state().heat_level,
            command_handler.game_state().reputation,
        ) {
            display_random_event(&event, ui.color_scheme()).await?;
            
            // Handle timed events
            if event.time_limit.is_some() {
                ui.color_scheme().print_warning("  [!] This event has a time limit!\n")?;
            }
        }
        
        // Apply heat decay
        let mut game_state = command_handler.game_state().clone();
        game_state.apply_heat_decay(0.99);
        command_handler.update_game_state(game_state);
        
        // Display command prompt
print!("\n");
prompt.display(ui.color_scheme())?;

// Get user input with proper debouncing
terminal::enable_raw_mode()?;
let mut input = String::new();

loop {
    // Use blocking read to get exactly one key event
    let key = loop {
        if let Ok(Event::Key(k)) = event::read() {
            break k;
        }
    };
    
    match key.code {
        KeyCode::Enter => {
            terminal::disable_raw_mode()?;
            println!();
            break;
        }
        KeyCode::Backspace => {
            if !input.is_empty() {
                input.pop();
                print!("\x08 \x08");
                io::stdout().flush()?;
            }
        }
        KeyCode::Char(c) => {
            input.push(c);
            print!("{}", c);
            io::stdout().flush()?;
        }
        KeyCode::Esc => {
            terminal::disable_raw_mode()?;
            input = "logout".to_string();
            println!();
            break;
        }
        _ => {}
    }
    
    // Clear any buffered events to prevent key repeat
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?;
    }
}
        
        // Process command
        let result = command_handler.execute(&input).await?;
        
        match result {
            commands::CommandResult::Logout => {
                auth_system.logout();
                show_logout_sequence(&user, ui.color_scheme()).await?;
                break;
            }
            commands::CommandResult::Exit => {
                break;
            }
            _ => {
                // Update user reputation in auth system
                auth_system.update_reputation(
                    command_handler.game_state().reputation - user.reputation
                )?;
            }
        }
        
        // Check if heat is critical
        if command_handler.game_state().heat_level >= 100.0 {
            show_busted_sequence(ui.color_scheme()).await?;
            auth_system.logout();
            break;
        }
    }
    
    Ok(())
}

/// Show welcome message after login
async fn show_welcome_message(user: &auth::User, color_scheme: &ui::ColorScheme) -> Result<()> {
    color_scheme.print_colored("\n")?;
    color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
    color_scheme.print_bright(&format!("    WELCOME BACK, AGENT {}\n", user.username.to_uppercase()))?;
    color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
    color_scheme.print_colored("\n")?;
    
    ui::animations::type_text_effect("Initializing encrypted channel...", 20, color_scheme).await?;
    println!();
    sleep(Duration::from_millis(500)).await;
    
    color_scheme.print_success("  [✓] Secure connection established\n")?;
    color_scheme.print_success("  [✓] Identity verified\n")?;
    color_scheme.print_success("  [✓] Access granted to CRIMSON-REDLINE network\n")?;
    
    println!();
    color_scheme.print_colored("  Last login: ")?;
    if let Some(last_login) = user.last_login {
        color_scheme.print_secondary(&format!("{}\n", last_login.format("%Y-%m-%d %H:%M:%S UTC")))?;
    } else {
        color_scheme.print_secondary("First login\n")?;
    }
    
    color_scheme.print_colored("  Reputation: ")?;
    color_scheme.print_bright(&format!("{}\n", user.reputation))?;
    
    color_scheme.print_colored("  Login count: ")?;
    color_scheme.print_secondary(&format!("{}\n", user.login_count))?;
    
    println!();
    color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
    color_scheme.print_dim("  Type 'help' for available commands\n")?;
    color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
    
    Ok(())
}

/// Display random event
async fn display_random_event(
    event: &game::events::RandomEvent,
    color_scheme: &ui::ColorScheme,
) -> Result<()> {
    println!();
    color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
    
    match event.severity {
        game::events::EventSeverity::Critical => {
            color_scheme.print_error(&format!("  [!!!] {} [!!!]\n", event.title))?;
        }
        game::events::EventSeverity::High => {
            color_scheme.print_warning(&format!("  [!!] {} [!!]\n", event.title))?;
        }
        game::events::EventSeverity::Medium => {
            color_scheme.print_bright(&format!("  [!] {} [!]\n", event.title))?;
        }
        game::events::EventSeverity::Low => {
            color_scheme.print_colored(&format!("  [*] {} [*]\n", event.title))?;
        }
    }
    
    color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
    color_scheme.print_secondary(&format!("  {}\n", event.description))?;
    println!();
    
    for (i, choice) in event.choices.iter().enumerate() {
        color_scheme.print_colored(&format!("  [{}] {}", i + 1, choice.label))?;
        
        if let Some(ref cost) = choice.cost {
            match cost {
                game::events::EventCost::Credits(c) => {
                    color_scheme.print_dim(&format!(" (Cost: {} credits)", c))?;
                }
                game::events::EventCost::Reputation(r) => {
                    color_scheme.print_dim(&format!(" (Cost: {} reputation)", r))?;
                }
                game::events::EventCost::Heat(h) => {
                    color_scheme.print_dim(&format!(" (Cost: {}% heat)", h))?;
                }
                game::events::EventCost::Time(t) => {
                    color_scheme.print_dim(&format!(" (Cost: {} seconds)", t))?;
                }
            }
        }
        println!();
    }
    
    color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
    
    Ok(())
}

/// Show logout sequence
async fn show_logout_sequence(user: &auth::User, color_scheme: &ui::ColorScheme) -> Result<()> {
    println!();
    color_scheme.print_colored("  [>] Initiating logout sequence...\n")?;
    sleep(Duration::from_millis(500)).await;
    
    color_scheme.print_colored("  [>] Saving session data...\n")?;
    sleep(Duration::from_millis(300)).await;
    
    color_scheme.print_colored("  [>] Clearing traces...\n")?;
    ui::animations::show_processing("Wiping logs", 1000).await?;
    
    color_scheme.print_colored("  [>] Disconnecting from CRIMSON-REDLINE network...\n")?;
    sleep(Duration::from_millis(500)).await;
    
    println!();
    color_scheme.print_success(&format!("  [✓] Agent {} successfully logged out\n", user.username))?;
    color_scheme.print_dim("  [*] Connection terminated\n")?;
    
    sleep(Duration::from_secs(2)).await;
    utils::clear_screen()?;
    
    Ok(())
}

/// Show exit sequence
async fn show_exit_sequence(color_scheme: &ui::ColorScheme) -> Result<()> {
    println!();
    ui::animations::glitch_transition(color_scheme).await?;
    
    color_scheme.print_colored("  [>] Shutting down CRIMSON-REDLINE system...\n")?;
    sleep(Duration::from_millis(500)).await;
    
    color_scheme.print_colored("  [>] Terminating all connections...\n")?;
    sleep(Duration::from_millis(300)).await;
    
    color_scheme.print_colored("  [>] Wiping temporary data...\n")?;
    sleep(Duration::from_millis(300)).await;
    
    color_scheme.print_colored("  [>] System shutdown complete.\n")?;
    println!();
    
    let goodbye = ui::ascii_art::create_box(40, 5, Some("GOODBYE"));
    for line in goodbye.lines() {
        color_scheme.print_dim(line)?;
        println!();
    }
    
    sleep(Duration::from_secs(1)).await;
    
    Ok(())
}

/// Show busted sequence when heat reaches 100%
async fn show_busted_sequence(color_scheme: &ui::ColorScheme) -> Result<()> {
    println!();
    color_scheme.print_error("═══════════════════════════════════════════════════════════════\n")?;
    color_scheme.print_error("                    !!! SYSTEM COMPROMISED !!!                 \n")?;
    color_scheme.print_error("═══════════════════════════════════════════════════════════════\n")?;
    
    ui::animations::show_error("HEAT LEVEL CRITICAL - LOCATION TRACED").await?;
    
    color_scheme.print_error("\n  [✗] Your location has been traced!\n")?;
    color_scheme.print_error("  [✗] Security forces have been dispatched!\n")?;
    color_scheme.print_error("  [✗] Emergency disconnect initiated!\n")?;
    
    sleep(Duration::from_secs(3)).await;
    
    ui::animations::glitch_transition(color_scheme).await?;
    
    Ok(())
}