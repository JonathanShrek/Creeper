use std::{
  io::Write, 
  path::PathBuf, 
  process::Command
};

pub fn send_email(item_path: &PathBuf) {
  let recipients = vec![
    "jshreckengost@itreconomics.com",
    //"kobrien@itreconomics.com",
    //"asievers@itreconomics.com",
    //"joe.beaulieu@itreconomics.com",
    //"bklardie@itreconomics.com"
  ];

  // Replace these values with your email content and recipient
  let to = recipients.join(", ");
  let subject = "Unknown PHP File Found and Removed";
  let body = format!("PHP file found at: {:?}", item_path); 

  // Run the sendmail command
  let result = Command::new("sendmail")
      .args(&["-t", "-i"])
      .stdin(std::process::Stdio::piped())
      .spawn();

  match result {
      Ok(mut child) => {
          // Write email content to the child process's stdin
          let email_content = format!("To: {}\nSubject: {}\n\n{}", to, subject, body);
          if let Some(mut stdin) = child.stdin.take() {
              stdin.write_all(email_content.as_bytes()).expect("Failed to write to stdin");
          }

          // Wait for the sendmail process to finish
          let status = child.wait().expect("Failed to wait for sendmail process");
          println!("Sendmail process exited with: {:?}", status);
      }
      Err(e) => {
          eprintln!("Failed to execute sendmail: {:?}", e);
      }
  }
}