# ojichat_rs_discordbot

[ojichat_rs](https://github.com/gamoutatsumi/ojichat_rs)を使用したDiscord Botです。

## 使い方

バイナリをそのまま実行する方法と、Dockerを使用する方法があります。

### バイナリそのまま

crates.ioには上げていないので、gitを通してインストールします。

Rust環境が必要です。

```bash
cargo install --git https://github.com/gamoutatsumi/ojichat_rs
```

環境変数にbotのトークンを入れる必要があります。

`.env` ファイルにも対応しています。実行する時のカレントディレクトリにファイルを配置してください。

```text
DISCORD_TOKEN=<botのトークン>
```

```bash
$ ojichat-rs-disbot
<bot-name> is connected!
```

### Docker経由

Docker Hubに上がってます。

相変わらず環境変数が必要なのでdocker-composeの利用をおすすめします。

今回は `--env-file` を使います。

```bash
docker pull gamoutatsumi/discord-ojibot:latest
docker run --rm --env-file=./.env gamoutatsumi/discord-ojibot:latest
<bot-name> is connected!
```

![地獄](./screenshot.png)
