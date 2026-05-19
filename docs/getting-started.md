---
title: Getting Started
---

# Getting Started

Install SkyBolt:

```bash
curl -fsSL https://agent.skylershuman.com/cli | sh
```

The installed command is `sky`.

Common first commands:

```bash
sky
sky --prompt "explain this project briefly"
sky provider login
sky list model
sky setup
```

SkyBolt reads local configuration, starts in the current working directory by default, and can also run against another directory with `sky --directory <path>`.

Some internal files and config paths still use the legacy `forge` name while the public CLI is `sky`.
