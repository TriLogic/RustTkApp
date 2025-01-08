use tcl::{tclfn, Interpreter, TclError, TclResult, Obj };
fn main() -> TclResult<()> {

    // create the interpreter
    let mut interp = Interpreter::new()?;

    // load Tk
    let tcl_result = interp.eval(r#"
        puts [string cat "TclTk Version:" [package require Tk]]
        puts [string cat "TclOO Version:" [package require TclOO]]
        "#)?;


    // Create a native Rust function that multiplies two numbers.
    let cmd_mul = tclfn!( &interp, /*cmd: "mul", args: "",*/
        fn mul( a: i32, b: i32 ) -> TclResult<i32> { 
            Ok( a * b )
        }
    );


    // Create a native Rust function that formats a greeting.
    let cmd_greetme = tclfn!(&interp, cmd: "greetme",/* args: String::from("name"),*/
        fn greetme( name: String ) -> TclResult<String> { 
            Ok(format!("Greetings {}!", name))
        }
    );

    // Create a basic Tk window with a couple of buttons.
    interp.eval(r#"

        wm title . "Main Window"

        proc on_button_click {} {
            tk_messageBox -message "Hello from Rust and Tcl/Tk!" -type ok
        }

        proc on_button_exit {} {
            set result [tk_messageBox -type okcancel -title "Verify App Exit" -message "Are you sure?" \
                -detail "You may have unsaved work that will be lost." -default cancel -icon question] 

            if {$result eq "ok"} {
                puts "Exiting application..."
                exit
            }
        }

        button .btn -text "Click Me!" -command on_button_click
        button .ext -text "Exit App" -command on_button_exit 
        
        pack .btn -padx 10 -pady 10
        pack .ext -padx 10 -pady 10
        
        wm protocol . WM_DELETE_WINDOW on_button_exit

        puts [greetme "World"]
        
        try {
            puts [greetme]
        } on error err {
            puts "Error: $err"
        }
    "#)?;


    let result = interp.eval("vwait forever")?;

    Ok(())
}

