# Shakespearean pokedex api

A simple pokedex rest api that returns the shakespearean description of the given pokemon.

Assuming Docker and docker-compose are installed on the machine, run the following command from the root of the project to build and launch the server: 

```
docker-compose up
```

**Usage:**
The server runs on localhost:3000
*Request* -> GET /pokemon/<pokemon_name>
*Response status codes* -> 202 | 400 | 404 | 429

External tools:
- https://courses.cs.washington.edu/courses/cse154/webservices/pokedex/ (For pokemon informations)
- https://funtranslations.com/api/shakespeare (For shakespeare translation | limited requests per hour/day!)