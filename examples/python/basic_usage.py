"""Example: Basic EWS client usage in Python.

注意: Python 绑定目前处于早期开发阶段。
此示例展示了计划中的 API,大部分功能尚未实现。
"""

# 当前可用: 仅客户端创建
from ews_client import EwsClient

# 创建客户端 (已实现)
try:
    client = EwsClient(
        endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
        username="user@example.com",
        password="password",
    )
    print("客户端创建成功!")
except ValueError as e:
    print(f"无效的端点 URL: {e}")
except RuntimeError as e:
    print(f"客户端创建失败: {e}")


# 以下是计划中的 API (尚未实现)
async def planned_api_example() -> None:
    """展示计划中的 API 用法"""
    client = EwsClient(
        endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
        username="user@example.com",
        password="password",
    )

    # 测试连接
    await client.check_connectivity()
    print("连接成功!")

    # 同步文件夹
    result = await client.sync_folder_hierarchy(sync_state=None)
    print(f"创建了 {len(result.created_folders)} 个文件夹")
    print(f"更新了 {len(result.updated_folders)} 个文件夹")
    print(f"删除了 {len(result.deleted_folder_ids)} 个文件夹")

    # 显示知名文件夹
    if result.well_known_folders:
        print("\n知名文件夹:")
        for name, folder_id in result.well_known_folders.items():
            print(f"  {name}: {folder_id}")

    # 显示前几个文件夹
    print("\n文件夹列表:")
    for folder in result.created_folders[:5]:
        print(f"  {folder.display_name} (未读: {folder.unread_count or 0}/{folder.total_count or 0})")

    # 如果有收件箱,同步其中的消息
    if result.well_known_folders and "inbox" in result.well_known_folders:
        inbox_id = result.well_known_folders["inbox"]
        print("\n同步收件箱消息...")
        message_result = await client.sync_messages(folder_id=inbox_id, sync_state=None)
        print(f"新消息: {len(message_result.created)}")
        print(f"更新: {len(message_result.updated)}")
        print(f"删除: {len(message_result.deleted)}")
        print(f"已读状态变更: {len(message_result.read_status_changed)}")


if __name__ == "__main__":
    print("=" * 60)
    print("EWS Client Python 示例")
    print("=" * 60)
    print("\n注意: 大部分功能尚未实现,仅展示计划中的 API\n")

    # 取消注释以运行计划中的 API 示例 (需要等待实现)
    # asyncio.run(planned_api_example())
