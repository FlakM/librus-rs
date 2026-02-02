use librus_rs::Client;

#[tokio::main]
async fn main() -> Result<(), librus_rs::Error> {
    println!("Authenticating...");
    let client = Client::from_env().await?;

    println!("Authentication successful!");

    // Test Me endpoint
    println!("\n--- Me ---");
    let me = client.me().await?;
    println!("{:#?}", me);

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

    println!("\nAll API tests completed!");
    Ok(())
}
