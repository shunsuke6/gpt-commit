# GPT-Commit

`git diff`を元にChatGPTにコミットメッセージを作成してもらうツールです。
個人で使用したものを公開用に修正したものです。

## なぜ使用するのか?

開発時にわかりやすいコミットメッセージのログがあると修正がしやすいが、適切な名前を考えるのが負荷。

機能追加がAdd、変更がmodify、バグ修正ががFixだらけになるのを防ぐ。

第一言語が英語じゃないため、AIに考えてもらうことで英語表現力の向上。

## 注意点

ChatGPTに3つの案を出力してもらい、その中から実際に使用するメッセージを選択します。

GPT4 推奨。
GPT-3.5 で動作を確認しているが精度は高くないと感じる。1/3が編集内容にそぐわ無いコミットメッセージで作成されることが多々ある。

適切なコミットを行うことで、コミットメッセージの出力もよくなります。

## セットアップ

```bash
cargo insatll gpt-commit
```

## 使い方

コミットする変更をステージングに上げ、gptcommitを実行する。

```bash
$ gptcommit run

Requesting to ChatGPT...
Please wait a moment.
# 実際の結果
# 1
# 2
# 3
Please choose [1-3]
```

APIの結果から良いと思うコミットのメッセージの番号を選択。

`git diff --cached`でなくワーキングエリアの内容全てをコミットする場合は`-n(--no-cached)`のオプションを使用する。
この場合`git commit -am "{choose message}"`が実行される。

```
gptcommit run -n
gptcommit run --no-cached
```

### 設定

以下設定を用意しています。

| 項目     | 内容                                   | オプション    |
| -------- | -------------------------------------- | ------------- |
| api_key  | ChatGPTのAPI_Key                       | -a,--api-key  |
| language | レスポンスの言語(commit messageは除く) | -l,--language |
| url      | ChatGPTのAPIのURL                      | -u,--url      |
| model    | ChatGPTのモデル                        | -m,--model    |

初回実行時にconfigファイルを作成するので、API_Keyを入力してください。

```bash
$ gptcommit config
Config file is not found
Create a Config file

Enter your ChatGPT API Key
{your API_Key}
```

初回実行時はでフォル値が設定される為、必要に応じてconfigコマンドを実行し設定する。

```bash
gptcommit config -l Japanese
```

```bash
# 引数に含めなくても実行可能
$ gptcommit config -l
Please enter language
> Japanese
```

## License

MIT

## Author

shunsuke6
