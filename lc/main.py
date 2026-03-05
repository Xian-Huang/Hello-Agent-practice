from langchain.agents import create_agent
from tools.emails import run_command, wolkdir
from langchain_deepseek import ChatDeepSeek
import os
from tools.emails import read_file

# 初始化聊天模型

# 设置API密钥（建议从环境变量读取）
os.environ["DEEPSEEK_API_KEY"] = "sk-2d41872df36c45a19311a53ab32aa1fe"

# 初始化模型
llm = ChatDeepSeek(
    model="deepseek-chat",
    temperature=0.7,
    max_tokens=1024,
)

agent = create_agent(
    model=llm,
    tools=[wolkdir,read_file,run_command],
    system_prompt="你是一个乐于助人的助手",
)



for chunk in agent.stream(  # [!code highlight]
    {"messages": [{"role": "user", "content": "浏览目录F:\Hello-Agent-practice\lc\data"},{"role": "user", "content": "读取每个文件的内容"},{"role": "user", "content": "运行shell命令dir"}]},
    stream_mode="updates",
):
    for step, data in chunk.items():
        print(f"step: {step}")
        print(f"content: {data['messages'][-1].content_blocks}")

# response = {'messages': [HumanMessage(content='旧金山的天气怎么样', additional_kwargs={}, response_metadata={}, id='638ff3e0-abd4-44ed-8ec3-8af69c0f3adb'), AIMessage(content='', additional_kwargs={}, response_metadata={'model': 'qwen3:8b', 'created_at': '2026-03-05T05:33:59.9237223Z', 'done': True, 'done_reason': 'stop', 'total_duration': 134689164000, 'load_duration': 91717231400, 'prompt_eval_count': 142, 'prompt_eval_duration': 6459101400, 'eval_count': 138, 'eval_duration': 36234456700, 'logprobs': None, 'model_name': 'qwen3:8b', 'model_provider': 'ollama'}, id='lc_run--019cbc7b-06bf-70c3-aa3a-385734e521cd-0', tool_calls=[{'name': 'get_weather', 'args': {'city': 'San Francisco'}, 'id': '36068bdf-5f2f-4937-aeaf-7e34b385e13f', 'type': 'tool_call'}], invalid_tool_calls=[], usage_metadata={'input_tokens': 142, 'output_tokens': 138, 'total_tokens': 280}), ToolMessage(content='San Francisco总是阳光明媚！', name='get_weather', id='dd7b5658-8e73-45ee-aeeb-8ceb690c7334', tool_call_id='36068bdf-5f2f-4937-aeaf-7e34b385e13f'), AIMessage(content='旧金山现在天气晴朗，阳光明媚，是个非常适合外出的好天气！记得做好防晒哦～', additional_kwargs={}, response_metadata={'model': 'qwen3:8b', 'created_at': '2026-03-05T05:35:15.0801994Z', 'done': True, 'done_reason': 'stop', 'total_duration': 75130610400, 'load_duration': 125446900, 'prompt_eval_count': 181, 'prompt_eval_duration': 2173752000, 'eval_count': 245, 'eval_duration': 72407563300, 'logprobs': None, 'model_name': 'qwen3:8b', 'model_provider': 'ollama'}, id='lc_run--019cbc7d-150c-7480-ac5b-c3d47d38558b-0', tool_calls=[], invalid_tool_calls=[], usage_metadata={'input_tokens': 181, 'output_tokens': 245, 'total_tokens': 426})]}
