from openai import OpenAI
class OpenAICompatibleClient:
    """
    一个用于调用任何兼容OpenAI接口的LLM服务的客户端。
    """
    def __init__(self, model: str, base_url: str):
        self.model = model
        self.client = OpenAI(base_url=base_url,api_key="ollama")

    def generate(self, prompt: str, system_prompt: str) -> str:
        """调用LLM API来生成回应。"""
        print("正在调用大语言模型...")
        try:
            messages = [
            {'role': 'system', 'content': system_prompt},
            {'role': 'user', 'content': prompt}
            ]
            response = self.client.chat.completions.create(
                    model=self.model,
                    messages=messages,
                    stream=False
                    )
            answer = response.choices[0].message.content
            print("大语言模型响应成功。")
            return answer
        except Exception as e:
            print(f"调用LLM API时发生错误: {e}")
            return "错误:调用语言模型服务时出错。"
        
    
    def test(self):
        """
         测试是否正确连接到LLM服务。
        """
        try:
            response = self.client.models.list()
            print("成功连接到LLM服务，模型列表如下:")
            print(response)
            for model in response.data:
                print(f"- {model.id}")
        except Exception as e:
            print(f"连接LLM服务时发生错误: {e}")