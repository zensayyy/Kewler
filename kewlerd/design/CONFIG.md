# Configuration

**Kewler** is configured using a REST API, preferable with the respective command line interface.  

## Version 0.1.0

Features:

- support git repositories using a pull-based system
  - configurable https and ssh connections
  - configurable poll time
  - secrets for private repositories

### API

```yaml
GitRepo:
    apiVersion:
        description: kewler/v1
        type: string
    url: 
        description: http URL to the repository 
        type: string
    secret:
        decription: username and password, but optional for public repositories
        type: optional(string)
    interval: 
        description: Interval in seconds to check repository for new releases
        type: int
---
GitRepoUpdate:
    url: 
        description: http URL to the repository 
        type: string
    secret:
        decription: username and password
        type: optional(string)
    interval: 
        description: Interval in seconds to check repository for new releases
        type: optional(int)
```

```yaml
/git/add:
    post:
        summary: Add a new git repository to watch
    requestBody:
        content: application/json
        schema: GitRepo
    responses:
        "200": Ok
        "401": Unauthorized
        "500": Internal server error
/git/{repository_url}/remove:
    post:
        summary: Removes a git repository from the watch list
    responses:
        "200": Ok
        "401": Unauthorized
        "500": Internal server error
/git/update:
    post:  
        summary: Updates the GitRepo object identified by url in GitRepoUpdate
    requestBody:    
        content: application/json
        schema: GitRepoUpdate
    responses: 
        "200": Ok
        "401": Unauthorized
        "500": Internal server error
```
