// Example: Basic EWS client usage in Rust
use ews_client_core::{Credentials, EwsClient};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
    let credentials = Credentials::basic("user@example.com", "password");
    let client = EwsClient::new(endpoint, credentials)?;

    // 测试连接
    println!("测试连接...");
    client.check_connectivity().await?;
    println!("连接成功!");

    // 检查是否为 Office365
    if client.is_office365() {
        println!("连接到 Office365 服务器");
    }

    // 同步文件夹层次结构
    println!("\n同步文件夹...");
    let folder_result = client.sync_folder_hierarchy(None).await?;
    println!("创建了 {} 个文件夹", folder_result.created_folders.len());
    println!("更新了 {} 个文件夹", folder_result.updated_folders.len());
    println!("删除了 {} 个文件夹", folder_result.deleted_folder_ids.len());

    // 显示知名文件夹
    if let Some(well_known) = &folder_result.well_known_folders {
        println!("\n知名文件夹:");
        for (id, name) in well_known {
            println!("  {}: {}", name, id);
        }
    }

    // 显示前几个文件夹
    println!("\n文件夹列表:");
    for folder in folder_result.created_folders.iter().take(5) {
        println!(
            "  {} (未读: {}/{})",
            folder.display_name,
            folder.unread_count.unwrap_or(0),
            folder.total_count.unwrap_or(0)
        );
    }

    // 如果有收件箱,同步其中的消息
    if let Some(well_known) = &folder_result.well_known_folders {
        if let Some(inbox_id) = well_known.get("inbox") {
            println!("\n同步收件箱消息...");
            let message_result = client.sync_messages(inbox_id, None).await?;
            println!("新消息: {}", message_result.created.len());
            println!("更新: {}", message_result.updated.len());
            println!("删除: {}", message_result.deleted.len());
            println!("已读状态变更: {}", message_result.read_status_changed.len());
        }
    }

    Ok(())
}
