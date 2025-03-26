# TUS_YUURIKAI_SYSTEM 📦🔑

## 概要 ✨
TUS_YUURIKAI_SYSTEM は、システム認証を活用して安全にロッカーを借りることができるシステムです。  
シンプルな操作で利用者も管理者もストレスなく利用できるよう設計されています！ 🚀🔒

---
## Install 方法

### 1. Dockerイメージのビルド

プロジェクトフォルダをダウンロードします。
(git cloneを用いる例)
```sh
git clone [URL]
```

プロジェクトのルートディレクトリに移動します。
ルートディレクトリにある `Dockerfile` を利用して、Dockerイメージをビルドします。

```sh
docker build -t tus_yuurikai_system .
```

### 2. Dockerコンテナの起動

コンテナ同士の接続用にtus_network(仮称)を作成します
```sh
docker network create tus_network
```

ビルドしたイメージを用いて、Dockerコンテナ(tus_app(仮称))を起動します。例として、ホストのポート 8000 をコンテナの同一ポートにマッピングする場合は以下のように実行します。
```sh
docker run -dit --name tus_app --network tus_network -p 8000:8000 tus_yuurikai_system tail -f /dev/null
```
※ ポート番号は必要に応じて変更してください。

プロジェクトファイルをデータベースコンテナ内/home/フォルダにコピーします。
```sh
docker cp [フォルダ名] tus_app:/home/[フォルダ名]
```

### 3. react-scripts の手動インストール

アプリコンテナ内に接続します。
```sh
docker exec -it tus_app /bin/bash
```
プロジェクトのルートディレクトリ(/home/内)に移動し、
react-scripts の手動インストールをします。
(npmがインストールされていない場合)
```sh
apt install npm
```
```sh
cd /home/app/frontend
```
```sh
npm install react-scripts
```
```sh
exit
```

### 4. postgreのdockerコンテナ作成
データベース用のDockerコンテナ(db(仮称))を起動します。

your_user : （任意のUSER名）

your_password : （任意のパスワード）

your_db : （任意のデータベース名）

```sh
docker run --name db \
             -v webapi_mvp_db_data:/var/lib/postgresql/data \
             --network tus_network \
             -e POSTGRES_USER=your_user \
             -e POSTGRES_PASSWORD=your_password \
             -e POSTGRES_DB=your_db \
             -p 5432:5432 \
             postgres:15-bullseye
```
※ ポート番号は必要に応じて変更してください。

### 5. データベースURLを環境変数に追加
コンテナを起動します。
```sh
docker start db
```
```sh
docker start tus_app
```
アプリコンテナ内に入ります。
```sh
docker exec -it tus_app /bin/bash
```
データベースURLを環境変数に追加します。
```sh
export DATABASE_URL=postgres://your_user:your_password@db:5432/your_db
```
(アプリをコンテナ外で動かす場合)
```sh
export DATABASE_URL=postgres://your_user:your_password@localhost:5432/your_db
```

### 6. .env.sampleファイルを用いて.envファイルを作成し必要情報を記入
(2/24追記)
プロジェクト設定に必要な各種キーは、.env ファイルに記述します。
以下の手順で、サンプルファイルから.env ファイルを作成し、編集してください。

```sh
cd /home/app
```

.envファイルを作成します。
```sh
cp .env.sample .env
```
```sh
apt install nano
```
.envファイルを編集します。
```sh
nano .env
```
(テキストエディタはnano,vim等お好みのものを使用してください。)
echoで編集してもよい

### 7. diesel関連のセットアップ
その後、以下のようにdieselのセットアップをします。
```sh
diesel setup
```
```sh
diesel migration run
```
schema.rs をsrc/infrastructure フォルダに移動します。
```sh
mv src/schema.rs src/infrastructure/schema.rs
```
diesel.toml ファイル内のfile = ... 欄をfile = "src/infrastructure/schema.rs" に書き換えます。
```sh
sed -i 's|^file = .*|file = "src/infrastructure/schema.rs"|' diesel.toml
```

### 8. 初期化用csvファイルのコピー
初期化用csvファイルをコンテナ内のlockerテーブルにコピーします。
プロジェクトのルートディレクトリに'lockerdata.csv'があると仮定します。

アプリコンテナから抜けます。
```sh
exit
```
csvファイルをデータベースコンテナ内/tmp/フォルダにコピーします。
```sh
docker cp lockerdata.csv db:/tmp/lockerdata.csv
```
データベースコンテナ内のデータベースに接続します。
```sh
docker exec -it db psql -U your_user -d your_db
```
\copyコマンドを使ってlockerテーブルにコピーします。(COPY 1210と表示されたら成功)
```sh
\copy locker FROM '/tmp/lockerdata.csv' WITH csv
```
管理者用ログイン情報を登録します
(1.0.0)
```sh
INSERT INTO admin VALUES ('[自分で決めたユーザーネーム]','[自分で決めたパスワード]');
```
(パスワード修正後)
```sh
INSERT INTO admin VALUES ('[自分で決めたユーザーネーム]','[自分で決めたパスワードのハッシュ値]');
```
接続を解除します。
```sh
\q
```

### 9. run.sh の実行
アプリコンテナ内に再度接続します。
```sh
docker exec -it tus_app /bin/bash
```
```sh
cd /home/app
```
その後、run.sh を実行します。
```sh
./run.sh
```
このスクリプトは以下の処理を行います。

    frontend ディレクトリに移動してフロントエンドのビルド (npm run build) を実行
    ルートディレクトリに戻り、バックエンドを起動 (cargo run)
    
---


## ユーザのシステムの流れ 👤

1. **ロッカー空き検索** 🔍  
   - 「ロッカー空き検索」をクリックして、利用可能なロッカーを確認します。

2. **利用規約の確認** 📄  
   - 規約をしっかりと読み、「同意して次に進む」をクリックします。  
   - **必ず最後まで読んでください！** 「利用規約とプライバシーポリシーに同意します。」のチェックをお忘れなく ✅

3. **申請情報の入力** 📝  
   - **申請者**と**共同利用者**の**学籍番号**および**氏名**を正確に入力します。  
   - 特に学籍番号が正しいかどうか、十分に確認してください（指定した学籍番号に認証メールが届きます 📧）。

4. **入力内容の確認** 🔍  
   - 入力内容をよくチェックし、誤りがないか確認しましょう！

5. **次のステップへ** ➡️  
   - 「今後のステップを確認する」ボタンをクリックし、次の手順に進みます。

6. **メール認証** ✉️  
   - 各自の Outlook に移動し、申請者は送られてきたメール内の URL をクリックします。  
   - 認証に少し時間がかかる可能性があるので、しばらくお待ちください ⏳。  
   - 共同利用者もそれぞれメールの URL をクリックしてください。  
   - ※申請者は、共同利用者に認証メールが届いているかも必ず確認してください 👀。

7. **認証完了 & ロッカー指定** 🔑  
   - 申請者宛に認証完了メールが届いたら、使いたいロッカー番号を指定します。  
   - ※利用できるのは「空き」の状態のロッカーのみです！ 🚪

8. **最終確認と登録** ✅  
   - 指定したロッカー番号をもう一度確認し、情報が正しいことを確かめます。  
   - 「確認して登録」ボタンをクリックし、登録が完了します 🎉。

---

## 管理者のシステムの流れ 🛠️

1. **ログイン** 🔐  
   - ブラウザで `[ドメイン]/login` にアクセスし、ユーザ名とパスワードを入力します。  
   - 初めての利用の場合は、事前に管理者からユーザ名とパスワードをご確認ください 📋。

2. **ロッカー設定** ⚙️  
   - ログイン後、メニューから「ロッカー設定」を選択します。

3. **作業の選択** 🛠️  
   - 実施したい作業を選びます：  
     - **ロッカー利用者検索** 🔍  
     - **ロッカーリセット** 🔄

4. **検索・リセットの操作** 🖥️  
   - **ロッカー利用者検索の場合:**  
     希望する年度（必須）、姓、名、階数を入力して検索を行います。  
   - **ロッカーリセットの場合:**  
     リセットボタンを押し、消去用パスワードを入力して、ロッカーの状態を「空き」に強制的にリセットします。  
     - ※ロッカーが実際に使用されていない場合のみ操作してください。  
     - 管理者の許可を得た上で、消去用パスワードを受け取ってから実施します 🔑。

---

## 注意事項 ⚠️

- ユーザは、必ず正確な学籍番号と氏名を入力してください。  
  誤った情報では認証メールが届かない恐れがあります 📧❌。
- ロッカーリセットの操作は管理者の承認が必要です。  
  不正な操作やタイミングの誤りにご注意ください 🔒🚫。
- 利用規約とプライバシーポリシーは必ずご確認の上、同意してください。  
  同意しない場合は次のステップへ進むことができません 📜✅。

---

## 技術スタック

**バックエンド**  
- **言語:** Rust  
- **フレームワーク:** Rocket  
- **ORM:** Diesel (PostgreSQL)  
- **シリアライズ:** serde, serde_json  
- **APIドキュメント:** utoipa, utoipa-swagger-ui  
- **その他:** dotenv, chrono, uuid, lettre, jsonwebtoken, rocket_cors, reqwest, regex

**フロントエンド**  
- **言語:** TypeScript  
- **ライブラリ:** React, Ant Design  
- **状態管理/通信:** TanStack React Query, axios  
- **ルーティング:** react-router-dom  

**インフラ**  
- **コンテナ:** Docker (Dockerfile によりコンテナ化)

---


## お問い合わせ 📬

バグ報告、機能リクエスト等がございましたら、  
[Issue](https://github.com/Lit-innosence/tus_yuurikai_system/issues) にてご連絡ください。  

---

TUS_YUURIKAI_SYSTEM をご利用いただき、安全かつ快適にロッカーを管理しましょう！ 🎉🔑🚀
