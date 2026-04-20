If we try to categorize multiple different use cases of commands, we get the following categories: 

| Kind                        | Behaviour                                                    |
|-----------------------------|--------------------------------------------------------------|
| TextCommand                 | !command -> {template}                                       |
| Simple Command              | some function inbetween, like !coins                         |
| Conditional Command         | some function that has a failure message                     |
| Query Command (Aggregation) | function that aggregates responses for usages of the command |
| Timer                       | timer that executes the command without any message          |

That means we have to conclude that over all Kinds/usages of commands and templates, a template has 0..n source messages. 

We could handle this by making all contexts being executed on each source message. 
For convenience, we could also have an optional accessor to the first source message.

An advantage of this approach would be that we could use aggregation on all (text) commands
# Handling context over-fetching
Just because some templates want to know the coins for a user, that does not mean we should query that for all users every time. 

Possible solutions: 
- Lazy fetching
- template functions
- explicit declaration

## Lazy fetching
> Have functions on context objects which query the data
> Problem: Concatinating latency (e.g. twitch api) when it could be done in parallel
## Template Functions
> Similar to lazy fetching, but with extension functions inside the template, using the template system itself
## Explicit declaration
> User needs to explicitly add this object to the context for each message