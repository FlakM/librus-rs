use librus_rs::Client;

#[tokio::main]
async fn main() -> Result<(), librus_rs::Error> {
    println!("Authenticating with Librus...");
    let mut client = Client::from_env().await?;

    println!("Authentication successful!");

    // Test Me endpoint
    println!("\n--- Me ---");
    let me = client.me().await?;
    println!("User: {} {}", me.me.user.first_name, me.me.user.last_name);

    // Test Grades
    println!("\n--- Grades ---");
    let grades = client.grades().await?;
    println!("Total grades: {}", grades.grades.len());

    // Test Homeworks
    println!("\n--- Homeworks ---");
    let homeworks = client.homeworks().await?;
    println!("Total homeworks: {}", homeworks.homeworks.len());

    // Test Attendances
    println!("\n--- Attendances ---");
    let attendances = client.attendances().await?;
    println!("Total attendances: {}", attendances.attendances.len());

    // Test Messages API
    println!("\n--- Messages ---");
    let unread = client.unread_counts().await?;
    println!(
        "Unread inbox: {}, notes: {}, alerts: {}",
        unread.inbox, unread.notes, unread.alerts
    );

    let inbox = client.inbox_messages(1, 5).await?;
    println!("Inbox messages (first 5):");
    for msg in &inbox {
        let content = Client::decode_message_content(&msg.content).unwrap_or_default();
        let preview: String = content.chars().take(50).collect();
        println!(
            "  [{}] {} - {} ({}...)",
            msg.send_date, msg.sender_name, msg.topic, preview
        );
    }

    // Get full message detail
    if let Some(first_msg) = inbox.first() {
        println!("\n--- Message Detail ---");
        let detail = client.message(&first_msg.message_id).await?;
        let content = Client::decode_message_content(&detail.message).unwrap_or_default();
        println!("From: {}", detail.sender_name);
        println!("Subject: {}", detail.topic);
        println!("Content:\n{}", content);
        if !detail.attachments.is_empty() {
            println!("Attachments: {:?}", detail.attachments);
        }
    }

    println!("\nAll API tests completed successfully!");
    Ok(())
}
