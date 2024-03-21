## Overview


This Telegram bot offers a fun character-guessing game using OpenAI's GPT-4 API. Users can participate in daily challenges, guess characters, and gain access to the GPT-4 model.

## Features
- **Character Guessing Game:** Users can guess characters in daily challenges within Telegram groups. Each user gets 5 tries and can ask for 2 hints.
- **Flexible Character Schedule:** You can create a daily or weekly character schedule in the config file to keep the game interesting.
- **GPT-4 Context Customization:** You can customize how the bot interacts with users by adjusting the helper_prompt_template setting in the config file.
- **Token Limit Control:** You can control how many tokens are used per GPT-4 request with the gpt_tokens_per_request_limit setting. This helps manage costs and use of the OpenAI API efficiently.
- **User Engagement Tracking:** The bot uses a SQLite database to keep track of user attempts, questions, and wins.
- **Using Damerau-Levenshtein algorithm for `answer` command.**

## Characters-calendar setting up example

You can define character for 

```toml
[Calendar]
plan = [
    { date = "2024-03-21", prompt = "Character for the day" },
    { date = "2024-03-22", prompt = "Another character for the day" },
    # Add more dates and prompts as desired
]
```


## Commands
Bot has following commands : `answer`, `gpt`, `info`, `question`, `requests`, `winners`


## Installation

1. Install docker. [Official guide](https://docs.docker.com/engine/install/ubuntu/) 

2. Clone the repository
```bash
    git clone https://github.com/rosenthall/character_guessing
```
3. Build the docker image
```bash
    docker build -t character_guessing .
```
4. Run docker container
```bash
    docker run -d --name character_guessing character_guessing
```