from langchain.tools import tool
import os
import subprocess


# 浏览文件夹
@tool
def wolkdir(path:str)   :
    """浏览文件夹并返回所有文件的路径
    
    args:
        path: 文件夹路径
    """
    files = []
    for root, dir, filenames in os.walk(path):
        for file in filenames:
            print("Found files:",os.path.join(root,file))
            files.append(os.path.join(root,file))
    return files


@tool
def read_file(path:str)   :
    """读取文件内容
    
    args:
        path: 文件路径
    """
    print("Read file:",path)
    with open(path,'r',encoding='utf-8') as f:
        content = f.read()
    return content


@tool
def  run_command(command:str)   :
    """运行shell命令
    
    args:
        command: shell命令
    """
    result = subprocess.run(command, shell=True, capture_output=True, text=True)
    if result.returncode != 0:
        return f"Error: {result.stderr}"
    return result.stdout
