from langchain_ollama import ChatOllama
from langchain.agents import create_agent

# 初始化聊天模型
llm = ChatOllama(
    model="qwen3:8b",
    temperature=0.7,
    base_url="http://localhost:11434",
    num_predict=256,
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