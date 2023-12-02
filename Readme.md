A cli tool in rust to help with file based routing for nextjs and sveltekit. It scans the folders and
takes a note of the folders that have got specific keywords that are used by nextjs and sveltekit to make their routers.

It has the following functions:
- add  | adds a new route to the project by making the folder structure.
- init | initalizes the cli in a project and generates the required config.
- gen  | generates the routes from a given config file, that is in json.
- show | displays all the routes of the project by scanning the folders 

[flags]
- config
- mode
- --out
- --lang

