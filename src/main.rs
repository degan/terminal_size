use ncurses::*;

fn main() {
    // Initialize ncurses
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);

    // Get the window size
    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    // Clear the screen
    clear();

    // Create a border around the main window
    box_(stdscr(), 0, 0);

    // Display the window size
    mvprintw(max_y / 2 - 1, (max_x - 20) / 2, "Terminal Window Size");
    mvprintw(
        max_y / 2 + 1,
        (max_x - 20) / 2,
        &format!("{}x{}", max_x, max_y),
    );

    // Refresh the screen
    refresh();

    // Wait for user input
    getch();

    // End ncurses
    endwin();
}
