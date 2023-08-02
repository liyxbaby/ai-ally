# AI Ally v1

## Your locally-hosted, versatile companion for AI chatbot creation!

AI Ally is designed to serve as a robust, simple, and light solution for crafting AI chatbots right on your personal machine. No need for any external APIs or library installation. Once you have installed the appropriate binary file and your chosen model, you can use the tool in diverse ways: As a WebUI for interacting with Large Language Models (LLM), for role-playing with a custom AI character, or even as an API for your other AI chatbot-intensive projects.

The repository encompasses several unique features such as short-term memory, optional support for CUDA, OpenCL and Metal, long-term memory, dialogue tuning, time recognition, in-chat learning, the ability to function as a REST API, reading character cards and an intuitive WebUI for easy data editing,configuration editing, or to send, modify and delete messages.

## Installation
Please download the binary fitting your OS and device from [here](https://github.com/liyxbaby/ai-ally/releases/tag/1.0.0) (for instance **ai-ally-windows-cuda.exe**). Then install the given LLM model with a **.gguf** extension (for example [this one](https://huggingface.co/TheBloke/zephyr-7B-beta-GGUF/resolve/main/zephyr-7b-beta.Q4_K_M.gguf?download=true)). Next, launch the ai-ally binary file and visit **http://localhost:3000** to view AI Ally WebUI. Click on the **gear icon** on the website and navigate to **config**. Replace **Path to your Large Language Model (LLM)** with the path leading to the **.gguf** model on your drive. Once this is done, you can start interacting with your chatbot!

![webui screenshot](https://raw.githubusercontent.com/liyxbaby/ai-ally/main/public/webui_screenshot.png)

![webui screenshot](https://raw.githubusercontent.com/liyxbaby/ai-ally/main/public/webui_screenshot2.png)

![License](https://img.shields.io/github/license/liyxbaby/ai-ally)
![Downloads](https://img.shields.io/github/downloads/liyxbaby/ai-ally/total)

## Features
This repository includes:
- Local functioning - doesn't need other API keys for service which makes it completely free (except for power costs - your computer needs to operate somehow). It also doesn't need internet to function.
- High privacy - All chats are saved locally in SQLite database. Hence, your AI's characteristics and your conversations remain on your PC.
- [API](/docs/api_docs.md) - can act as a ba