 ## Handling Errors with Default Methods
 
 ```rust
 fn whisper_unwrap() {
     Whisper::new()
         .icon(IconKind::NfFaInfoCircle)
         .message("unwrap")
         .message("Returns the contained Ok value, consuming the self value,\
          function may panic, its use is generally discouraged")
         .whisper()
         .unwrap();
 }


 fn whisper_unwrap_or_else() {
     Whisper::new()
         .icon(IconKind::NfFaBug)
         .message("unwrap_or_else")
         .message("Unwrapping a `Whisper` instance or panicking with a custom message.")
         .whisper()
         .unwrap_or_else(|err| panic("Failed to print message: {}", err));
 }

 fn whisper_expect() {
     Whisper::new()
         .icon(IconKind::NfFaWarning)
         .message("expect")
         .message(
             "Returns the contained Ok value, consuming the self value.\
              Because this function may panic, its use is generally discouraged.\
              Instead, prefer to use pattern matching and handle the Err case explicitly,\
              or call unwrap_or, unwrap_or_else, or unwrap_or_default.",
         )
         .whisper()
         .expect("Failed to print message");
 }

 fn whisper_map_err() -> Result<(), Error> {
     Whisper::new()
         .icon(IconKind::NfFaTimes)
         .message("map_err")
         .message("Maps a Result<T, E> to Result<T, F> \
          by applying a function to a contained Err value, leaving an Ok value untouched.")
         .message("This function can be used to pass through a \
          successful result while handling an error.")
         .whisper()
         .map_err(|err| Error::new(ErrorKind::Other, err))?;
     Ok(())
 }

 fn whisper_map_err_2() -> Result<(), Error> {
     let err = "Hypothetical error message";
     Whisper::new()
         .icon(IconKind::NfFaTimes)
         .message(&format("Error executing command: {}", err))
         .whisper()
         .map_err(|_| Error::new(ErrorKind::Other, "Whisper failed"))?;
     Ok(())
 }
 fn whisper_ok() {
     Whisper::new()
         .icon(IconKind::NfFaTimes)
         .message("ok")
         .message("Converts from Result<T, E> to Option<T>.")
         .message("consuming self, and discarding the error, if any.")
         .whisper()
         .ok();
 }


 fn whisper_box_dyn_error() -> Result<(), Box<dyn std::error::Error>> {
    Whisper::new()
        .icon(IconKind::NfFaTimes)
       .message("box_dyn_error")
       .message("This function returns a Result. If the operation is successful,\
        it returns Ok(()).")
       .message("If there is an error during the operation, it returns WhisperError.")
       .whisper()?;
    Ok(())
 }
 fn whisper_match() {
     let whisper = Whisper::new()
         .icon(IconKind::NfFaBug)
         .message("match");

     match whisper.whisper() {
         Ok(()) => println("Message printed successfully"),
         Err(error) => eprintln("Failed to print message: {error}",),
     }
 }

 fn whisper_if_let() {
     let whisper = Whisper::new()
         .icon(IconKind::NfFaBug)
         .message("if_let")
         .message("test_whisper_if_let");

     if let Err(error) = whisper.whisper() {
         eprintln("Failed to print message: {error}",);
     }
 }

 fn whisper_execute_command_example(command: &str, args: &[&str]) -> Result<(), Error> {
     let output = std::process::Command::new(command)
         .args(args)
         .output()?;

     let whisper = |message: &str, icon: IconKind| {
         Whisper::new()
             .icon(icon)
             .message(message)
             .whisper()
             .ok();
     };

     if output.status.success() {
         let command = format("{} {}", command, args.join(" "));
         whisper(&command, IconKind::NfFaRefresh);
     } else {
         whisper("Failed to execute command", IconKind::NfFaTimes);
     };

     Ok(())
 }
 ```

