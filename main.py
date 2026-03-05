from langchain.agents import create_agent
import os
from langchain_deepseek import ChatDeepSeek

# 设置API密钥（建议从环境变量读取）
os.environ["DEEPSEEK_API_KEY"] = "sk-2d41872df36c45a19311a53ab32aa1fe"


# 初始化聊天模型
llm = ChatDeepSeek(
    model="deepseek-chat",  # 或其他DeepSeek模型
    temperature=0.7,
    max_tokens=1024,
)

def get_weather(city: str) -> str:
    """获取指定城市的天气。"""
    return f"{city}总是阳光明媚！"

agent = create_agent(
    model=llm,
    tools=[get_weather],
    system_prompt="你是一个乐于助人的助手",
)

# 运行代理
response = agent.invoke(
    {"messages": [{"role": "user", "content": "旧金山的天气怎么样"}]}
)

print(response)