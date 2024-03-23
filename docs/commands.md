## create

`$ create <WORKSPACE_NAME>`

Create an empty workspace with name `<WORKSPACE_NAME>`.

## list

`$ list`

Lists all workspaces.

## delete

`> delete <WORKSPACE_NAME>`

Deletes workspace with name `<WORKSPACE_NAME>`.

`> delete --all`

Delete all workspaces and embedded documents.

## import

`> import --source zotero|folder|item <SOURCE_IDENTIFIER>`

Import data from Zotero, folder or item to workspace with name `<SOURCE>-<WORKSPACE_NAME>`.

e.g. `ayda import --source-type folder /Users/richardlyon/InterestingDocs`

## chat

`> chat <WORKSPACE_NAME>`

Chat with workspace `<WORKSPACE_NAME>`. Chat uses Large Language Model general knowledge together with the documents in the workspace to produce output, and rolling chat history.

To change mode, type `/query`.

To exit, type `/exit`.

## query

`> query <WORKSPACE_NAME>`

Query workspace `<WORKSPACE_NAME>`. Querying does not use the Large Language Model unless there are relevant sources and does not recall chat history.

To change mode, type `/chat`.

To exit, type `/exit`.

## config

`> config`

Configure the application.
