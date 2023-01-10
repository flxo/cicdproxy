# CICD Proxy

~/.gitconfig

```
[credential "https://git.hub.vwgroup.com"]
	helper = !/usr/bin/gh auth git-credential
[http "https://git.hub.vwgroup.com/"]
  proxy = http://host:44787
[http "https://jfrog.hub.vwgroup.com/"]
  proxy = http://host:44787
```

/etc/hosts

```
10.37.129.2	host
```

## ssh

```
ssh -R 44787:localhost:44787 northstar
```