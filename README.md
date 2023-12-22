# QDrant 2023 Hackaton

This is a repository for QDrant 2023 Hackaton crated by [Ivan Pleshkov](https://https://github.com/IvanPleshkov).

# Table of Contents
1. [Project description](#Project-description)
2. [How to play](#How-to-play)
3. [How it works](#How-it-works)
4. [Define issue for real qdrant usage](#Define-issue-for-real-qdrant-usage)
5. [Using discovery search to analyse players promts](#Using-discovery-search-to-analyse-players-promts)
6. [Conclusion](#Conclusion)

## Project description

This project presents a simple quest game where player can use text promts to interact with the game. Screenshot:

![Screenshot](images/start_screenshot.jpg)

The main idea of this project is to use QDrant to analyse players promts and find out what they are looking for in the game. More details about this idea you can find in [Define issue for real qdrant usage](#Define-issue-for-real-qdrant-usage) section.

To build and run this project you need first to install [Rust](https://www.rust-lang.org/tools/install). And then, in the project directory, run:

```bash
cargo run --release
```

## How to play

Your promts are addressed to Andrey.

Just type your promt in textbox and wait for the result. See the actual action by this promt in logs. Sorry, but there is no any visualisation of the promt recognising result yet.

There are pure amout of actions and no game ending condition.

You can ask andrey to change this postion. For instance, by request `move to Luis` Andrey will go to Luis:

![GoToLuis](images/go_to_luis.jpg)

Also, Andrey can grab and drop items. For instance, by request `take bottle` Andrey will grab the bottle:

![TakeBottle](images/take_bottle.jpg)

And by request `give it to Roman` Andrey will drop the bottle near roman:

![DropBottle](images/drop_bottle.jpg)

Also Andrey can take/drop cups. That's it, no more actions here.

## How it works

For this game I used QDrant and [Cohere](https://cohere.ai/) for text embeddings.



## Define issue for real qdrant usage

## Using discovery search to analyse players promts

## Conclusion

For this short period of time I've managed to create a simple quest game where player can use text promts to interact with the game. Also, I've managed to use QDrant to analyse players promts and find out what they are looking for in the game. I think this is a good example of how QDrant can be used in real life. I hope you liked it.
