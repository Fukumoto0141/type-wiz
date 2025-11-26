/*
 * src/questions.rs
 * お題データを管理するモジュール
 */

// `pub` をつけて、他のファイル (main.rs) から見えるようにする
/*
 * src/questions.rs
 * (romaji -> hiragana に変更)
 */

// 構造体のフィールド名を変更
#[derive(Copy, Clone)]
pub struct Question {
    pub japanese: &'static str, // 表示用 (漢字混じり)
    pub hiragana: &'static str, // タイピング用 (ひらがな)
}

/// 問題リスト (ひらがなの文字数昇順)
pub const QUESTIONS_LIST: &'static [Question] = &[
    // --- 都道府県・地名 (Geography) ---
    Question { japanese: "北海道", hiragana: "ほっかいどう" },
    Question { japanese: "青森県", hiragana: "あおもりけん" },
    Question { japanese: "岩手県", hiragana: "いわてけん" },
    Question { japanese: "宮城県", hiragana: "みやぎけん" },
    Question { japanese: "秋田県", hiragana: "あきたけん" },
    Question { japanese: "山形県", hiragana: "やまがたけん" },
    Question { japanese: "福島県", hiragana: "ふくしまけん" },
    Question { japanese: "茨城県", hiragana: "いばらきけん" },
    Question { japanese: "栃木県", hiragana: "とちぎけん" },
    Question { japanese: "群馬県", hiragana: "ぐんまけん" },
    Question { japanese: "埼玉県", hiragana: "さいたまけん" },
    Question { japanese: "千葉県", hiragana: "ちばけん" },
    Question { japanese: "東京都", hiragana: "とうきょうと" },
    Question { japanese: "神奈川県", hiragana: "かながわけん" },
    Question { japanese: "新潟県", hiragana: "にいがたけん" },
    Question { japanese: "富山県", hiragana: "とやまけん" },
    Question { japanese: "石川県", hiragana: "いしかわけん" },
    Question { japanese: "福井県", hiragana: "ふくいけん" },
    Question { japanese: "山梨県", hiragana: "やまなしけん" },
    Question { japanese: "長野県", hiragana: "ながのけん" },
    Question { japanese: "岐阜県", hiragana: "ぎふけん" },
    Question { japanese: "静岡県", hiragana: "しずおかけん" },
    Question { japanese: "愛知県", hiragana: "あいちけん" },
    Question { japanese: "三重県", hiragana: "みえけん" },
    Question { japanese: "滋賀県", hiragana: "しがけん" },
    Question { japanese: "京都府", hiragana: "きょうとふ" },
    Question { japanese: "大阪府", hiragana: "おおさかふ" },
    Question { japanese: "兵庫県", hiragana: "ひょうごけん" },
    Question { japanese: "奈良県", hiragana: "ならけん" },
    Question { japanese: "和歌山県", hiragana: "わかやまけん" },
    Question { japanese: "鳥取県", hiragana: "とっとりけん" },
    Question { japanese: "島根県", hiragana: "しまねけん" },
    Question { japanese: "岡山県", hiragana: "おかやまけん" },
    Question { japanese: "広島県", hiragana: "ひろしまけん" },
    Question { japanese: "山口県", hiragana: "やまぐちけん" },
    Question { japanese: "徳島県", hiragana: "とくしまけん" },
    Question { japanese: "香川県", hiragana: "かがわけん" },
    Question { japanese: "愛媛県", hiragana: "えひめけん" },
    Question { japanese: "高知県", hiragana: "こうちけん" },
    Question { japanese: "福岡県", hiragana: "ふくおかけん" },
    Question { japanese: "佐賀県", hiragana: "さがけん" },
    Question { japanese: "長崎県", hiragana: "ながさきけん" },
    Question { japanese: "熊本県", hiragana: "くまもとけん" },
    Question { japanese: "大分県", hiragana: "おおいたけん" },
    Question { japanese: "宮崎県", hiragana: "みやざきけん" },
    Question { japanese: "鹿児島県", hiragana: "かごしまけん" },
    Question { japanese: "沖縄県", hiragana: "おきなわけん" },
    Question { japanese: "富士山", hiragana: "ふじさん" },
    Question { japanese: "日本列島", hiragana: "にほんれっとう" },
    Question { japanese: "太平洋", hiragana: "たいへいよう" },

    // --- 動物・自然 (Animals & Nature) ---
    Question { japanese: "象", hiragana: "ぞう" },
    Question { japanese: "麒麟", hiragana: "きりん" },
    Question { japanese: "ライオン", hiragana: "らいおん" },
    Question { japanese: "パンダ", hiragana: "ぱんだ" },
    Question { japanese: "ウサギ", hiragana: "うさぎ" },
    Question { japanese: "亀", hiragana: "かめ" },
    Question { japanese: "ペンギン", hiragana: "ぺんぎん" },
    Question { japanese: "イルカ", hiragana: "いるか" },
    Question { japanese: "クジラ", hiragana: "くじら" },
    Question { japanese: "タカ", hiragana: "たか" },
    Question { japanese: "ワシ", hiragana: "わし" },
    Question { japanese: "フクロウ", hiragana: "ふくろう" },
    Question { japanese: "ひまわり", hiragana: "ひまわり" },
    Question { japanese: "バラ", hiragana: "ばら" },
    Question { japanese: "タンポポ", hiragana: "たんぽぽ" },
    Question { japanese: "朝顔", hiragana: "あさがお" },
    Question { japanese: "紅葉", hiragana: "こうよう" },
    Question { japanese: "雪だるま", hiragana: "ゆきだるま" },
    Question { japanese: "台風", hiragana: "たいふう" },
    Question { japanese: "地震", hiragana: "じしん" },
    Question { japanese: "雷", hiragana: "かみなり" },
    Question { japanese: "虹", hiragana: "にじ" },
    Question { japanese: "満月", hiragana: "まんげつ" },
    Question { japanese: "星空", hiragana: "ほしぞら" },
    Question { japanese: "宇宙", hiragana: "うちゅう" },
    Question { japanese: "銀河", hiragana: "ぎんが" },
    Question { japanese: "砂漠", hiragana: "さばく" },
    Question { japanese: "森林", hiragana: "しんりん" },

    // --- 食べ物・飲み物 (Food & Drink) ---
    Question { japanese: "おにぎり", hiragana: "おにぎり" },
    Question { japanese: "味噌汁", hiragana: "みそしる" },
    Question { japanese: "納豆", hiragana: "なっとう" },
    Question { japanese: "卵焼き", hiragana: "たまごやき" },
    Question { japanese: "焼き魚", hiragana: "やきざかな" },
    Question { japanese: "カレーライス", hiragana: "かれーらいす" },
    Question { japanese: "ハンバーグ", hiragana: "はんばーぐ" },
    Question { japanese: "スパゲッティ", hiragana: "すぱげってぃ" },
    Question { japanese: "ピザ", hiragana: "ぴざ" },
    Question { japanese: "サンドイッチ", hiragana: "さんどいっち" },
    Question { japanese: "オムライス", hiragana: "おむらいす" },
    Question { japanese: "天ぷら", hiragana: "てんぷら" },
    Question { japanese: "そば", hiragana: "そば" },
    Question { japanese: "うどん", hiragana: "うどん" },
    Question { japanese: "お好み焼き", hiragana: "おこのみやき" },
    Question { japanese: "たこ焼き", hiragana: "たこやき" },
    Question { japanese: "餃子", hiragana: "ぎょうざ" },
    Question { japanese: "チャーハン", hiragana: "ちゃーはん" },
    Question { japanese: "麻婆豆腐", hiragana: "まーぼーどうふ" },
    Question { japanese: "エビチリ", hiragana: "えびちり" },
    Question { japanese: "ショートケーキ", hiragana: "しょーとけーき" },
    Question { japanese: "チョコレート", hiragana: "ちょこれーと" },
    Question { japanese: "プリン", hiragana: "ぷりん" },
    Question { japanese: "アイスクリーム", hiragana: "あいすくりーむ" },
    Question { japanese: "和菓子", hiragana: "わがし" },
    Question { japanese: "コーヒー", hiragana: "こーひー" },
    Question { japanese: "紅茶", hiragana: "こうちゃ" },
    Question { japanese: "緑茶", hiragana: "りょくちゃ" },
    Question { japanese: "コーラ", hiragana: "こーら" },
    Question { japanese: "オレンジジュース", hiragana: "おれんじじゅーす" },
    Question { japanese: "水", hiragana: "みず" },
    Question { japanese: "牛乳", hiragana: "ぎゅうにゅう" },

    // --- 日用品・家具・家電 (Daily Items) ---
    Question { japanese: "スマートフォン", hiragana: "すまーとふぉん" },
    Question { japanese: "テレビ", hiragana: "てれび" },
    Question { japanese: "冷蔵庫", hiragana: "れいぞうこ" },
    Question { japanese: "洗濯機", hiragana: "せんたくき" },
    Question { japanese: "電子レンジ", hiragana: "でんしれんじ" },
    Question { japanese: "掃除機", hiragana: "そうじき" },
    Question { japanese: "エアコン", hiragana: "えあこん" },
    Question { japanese: "パソコン", hiragana: "ぱそこん" },
    Question { japanese: "キーボード", hiragana: "きーぼーど" },
    Question { japanese: "マウス", hiragana: "まうす" },
    Question { japanese: "時計", hiragana: "とけい" },
    Question { japanese: "財布", hiragana: "さいふ" },
    Question { japanese: "鍵", hiragana: "かぎ" },
    Question { japanese: "眼鏡", hiragana: "めがね" },
    Question { japanese: "傘", hiragana: "かさ" },
    Question { japanese: "靴", hiragana: "くつ" },
    Question { japanese: "帽子", hiragana: "ぼうし" },
    Question { japanese: "机", hiragana: "つくえ" },
    Question { japanese: "椅子", hiragana: "いす" },
    Question { japanese: "ベッド", hiragana: "べっど" },
    Question { japanese: "本棚", hiragana: "ほんだな" },
    Question { japanese: "鏡", hiragana: "かがみ" },
    Question { japanese: "タオル", hiragana: "たおる" },
    Question { japanese: "石鹸", hiragana: "せっけん" },
    Question { japanese: "歯ブラシ", hiragana: "はぶらし" },

    // --- 学校・勉強 (School & Study) ---
    Question { japanese: "先生", hiragana: "せんせい" },
    Question { japanese: "生徒", hiragana: "せいと" },
    Question { japanese: "教室", hiragana: "きょうしつ" },
    Question { japanese: "黒板", hiragana: "こくばん" },
    Question { japanese: "教科書", hiragana: "きょうかしょ" },
    Question { japanese: "ノート", hiragana: "のーと" },
    Question { japanese: "鉛筆", hiragana: "えんぴつ" },
    Question { japanese: "消しゴム", hiragana: "けしごむ" },
    Question { japanese: "定規", hiragana: "じょうぎ" },
    Question { japanese: "宿題", hiragana: "しゅくだい" },
    Question { japanese: "テスト", hiragana: "てすと" },
    Question { japanese: "受験", hiragana: "じゅけん" },
    Question { japanese: "合格", hiragana: "ごうかく" },
    Question { japanese: "卒業", hiragana: "そつぎょう" },
    Question { japanese: "入学式", hiragana: "にゅうがくしき" },
    Question { japanese: "運動会", hiragana: "うんどうかい" },
    Question { japanese: "文化祭", hiragana: "ぶんかさい" },
    Question { japanese: "修学旅行", hiragana: "しゅうがくりょこう" },
    Question { japanese: "部活動", hiragana: "ぶかつどう" },
    Question { japanese: "給食", hiragana: "きゅうしょく" },
    Question { japanese: "算数", hiragana: "さんすう" },
    Question { japanese: "数学", hiragana: "すうがく" },
    Question { japanese: "国語", hiragana: "こくご" },
    Question { japanese: "理科", hiragana: "りか" },
    Question { japanese: "社会", hiragana: "しゃかい" },
    Question { japanese: "英語", hiragana: "えいご" },
    Question { japanese: "体育", hiragana: "たいいく" },
    Question { japanese: "音楽", hiragana: "おんがく" },
    Question { japanese: "美術", hiragana: "びじゅつ" },
    Question { japanese: "歴史", hiragana: "れきし" },

    // --- 感情・状態 (Emotions & States) ---
    Question { japanese: "嬉しい", hiragana: "うれしい" },
    Question { japanese: "楽しい", hiragana: "たのしい" },
    Question { japanese: "悲しい", hiragana: "かなしい" },
    Question { japanese: "寂しい", hiragana: "さびしい" },
    Question { japanese: "面白い", hiragana: "おもしろい" },
    Question { japanese: "難しい", hiragana: "むずかしい" },
    Question { japanese: "簡単", hiragana: "かんたん" },
    Question { japanese: "大好き", hiragana: "だいすき" },
    Question { japanese: "大切", hiragana: "たいせつ" },
    Question { japanese: "本気", hiragana: "ほんき" },
    Question { japanese: "勇気", hiragana: "ゆうき" },
    Question { japanese: "希望", hiragana: "きぼう" },
    Question { japanese: "夢", hiragana: "ゆめ" },
    Question { japanese: "努力", hiragana: "どりょく" },
    Question { japanese: "成功", hiragana: "せいこう" },
    Question { japanese: "失敗", hiragana: "しっぱい" },
    Question { japanese: "挑戦", hiragana: "ちょうせん" },
    Question { japanese: "自由", hiragana: "じゆう" },
    Question { japanese: "責任", hiragana: "せきにん" },
    Question { japanese: "信頼", hiragana: "しんらい" },
    Question { japanese: "約束", hiragana: "やくそく" },
    Question { japanese: "感謝", hiragana: "かんしゃ" },
    Question { japanese: "感動", hiragana: "かんどう" },
    Question { japanese: "緊張", hiragana: "きんちょう" },
    Question { japanese: "安心", hiragana: "あんしん" },

    // --- 四字熟語 (Four-Character Idioms) ---
    Question { japanese: "一石二鳥", hiragana: "いっせきにちょう" },
    Question { japanese: "一日一善", hiragana: "いちにちいちぜん" },
    Question { japanese: "三日坊主", hiragana: "みっかぼうず" },
    Question { japanese: "十人十色", hiragana: "じゅうにんといろ" },
    Question { japanese: "自業自得", hiragana: "じごうじとく" },
    Question { japanese: "弱肉強食", hiragana: "じゃくにくきょうしょく" },
    Question { japanese: "一心不乱", hiragana: "いっしんふらん" },
    Question { japanese: "温故知新", hiragana: "おんこちしん" },
    Question { japanese: "花鳥風月", hiragana: "かちょうふうげつ" },
    Question { japanese: "起死回生", hiragana: "きしかいせい" },
    Question { japanese: "急転直下", hiragana: "きゅうてんちょっか" },
    Question { japanese: "言行一致", hiragana: "げんこういっち" },
    Question { japanese: "才色兼備", hiragana: "さいしょくけんび" },
    Question { japanese: "山紫水明", hiragana: "さんしすいめい" },
    Question { japanese: "四面楚歌", hiragana: "しめんそか" },
    Question { japanese: "初志貫徹", hiragana: "しょしかんてつ" },
    Question { japanese: "誠心誠意", hiragana: "せいしんせいい" },
    Question { japanese: "千載一遇", hiragana: "せんざいいちぐう" },
    Question { japanese: "大器晩成", hiragana: "たいきばんせい" },
    Question { japanese: "単刀直入", hiragana: "たんとうちょくにゅう" },
    Question { japanese: "猪突猛進", hiragana: "ちょとつもうしん" },
    Question { japanese: "電光石火", hiragana: "でんこうせっか" },
    Question { japanese: "日進月歩", hiragana: "にっしんげっぽ" },
    Question { japanese: "半信半疑", hiragana: "はんしんはんぎ" },
    Question { japanese: "粉骨砕身", hiragana: "ふんこつさいしん" },
    Question { japanese: "本末転倒", hiragana: "ほんまつてんとう" },
    Question { japanese: "無我夢中", hiragana: "むがむちゅう" },
    Question { japanese: "油断大敵", hiragana: "ゆだんたいてき" },
    Question { japanese: "臨機応変", hiragana: "りんきおうへん" },

    // --- MARK:カタカナ語・ビジネス・IT (Katakana/Tech) ---
    Question { japanese: "インターネット", hiragana: "いんたーねっと" },
    Question { japanese: "ウェブサイト", hiragana: "うぇぶさいと" },
    Question { japanese: "アプリケーション", hiragana: "あぷりけーしょん" },
    Question { japanese: "ダウンロード", hiragana: "だうんろーど" },
    Question { japanese: "アップロード", hiragana: "あっぷろーど" },
    Question { japanese: "ログイン", hiragana: "ろぐいん" },
    Question { japanese: "ログアウト", hiragana: "ろぐあうと" },
    Question { japanese: "パスワード", hiragana: "ぱすわーど" },
    Question { japanese: "アカウント", hiragana: "あかうんと" },
    Question { japanese: "プロフィール", hiragana: "ぷろふぃーる" },
    Question { japanese: "コメント", hiragana: "こめんと" },
    Question { japanese: "シェア", hiragana: "しぇあ" },
    Question { japanese: "フォロー", hiragana: "ふぉろー" },
    Question { japanese: "ブロック", hiragana: "ぶろっく" },
    Question { japanese: "通知", hiragana: "つうち" },
    Question { japanese: "設定", hiragana: "せってい" },
    Question { japanese: "検索", hiragana: "けんさく" },
    Question { japanese: "履歴", hiragana: "りれき" },
    Question { japanese: "クリエイティブ", hiragana: "くりえいてぃぶ" },
    Question { japanese: "コミュニケーション", hiragana: "こみゅにけーしょん" },
    Question { japanese: "プレゼンテーション", hiragana: "ぷれぜんてーしょん" },
    Question { japanese: "モチベーション", hiragana: "もちべーしょん" },
    Question { japanese: "イノベーション", hiragana: "いのべーしょん" },
    Question { japanese: "マーケティング", hiragana: "まーけてぃんぐ" },
    Question { japanese: "マネジメント", hiragana: "まねじめんと" },
    Question { japanese: "リーダーシップ", hiragana: "りーだーしっぷ" },
    Question { japanese: "グローバル", hiragana: "ぐろーばる" },
    Question { japanese: "サステナブル", hiragana: "さすてなぶる" },
    Question { japanese: "ダイバーシティ", hiragana: "だいばーしてぃ" },
    Question { japanese: "コンプライアンス", hiragana: "こんぷらいあんす" },
    Question { japanese: "エビデンス", hiragana: "えびでんす" },
    Question { japanese: "アジェンダ", hiragana: "あじぇんだ" },
    Question { japanese: "タスク", hiragana: "たすく" },
    Question { japanese: "リスク", hiragana: "りすく" },
    Question { japanese: "メリット", hiragana: "めりっと" },
    Question { japanese: "デメリット", hiragana: "でめりっと" },
    Question { japanese: "コスト", hiragana: "こすと" },
    Question { japanese: "パフォーマンス", hiragana: "ぱふぉーまんす" },
    Question { japanese: "フィードバック", hiragana: "ふぃーどばっく" },
    Question { japanese: "ブラウザ", hiragana: "ぶらうざ" },
    Question { japanese: "インストール", hiragana: "いんすとーる" },
    Question { japanese: "アップデート", hiragana: "あっぷでーと" },
    Question { japanese: "ウイルス", hiragana: "ういるす" },
    Question { japanese: "ファイアウォール", hiragana: "ふぁいあうぉーる" },
    Question { japanese: "バックアップ", hiragana: "ばっくあっぷ" },
    Question { japanese: "リカバリー", hiragana: "りかばりー" },
    Question { japanese: "ショートカットキー", hiragana: "しょーとかっときー" },
    Question { japanese: "ディスプレイ", hiragana: "でぃすぷれい" },
    Question { japanese: "プロジェクター", hiragana: "ぷろじぇくたー" },
    Question { japanese: "タブレット", hiragana: "たぶれっと" },
    Question { japanese: "バッテリー", hiragana: "ばってりー" },
    Question { japanese: "充電器", hiragana: "じゅうでんき" },
    Question { japanese: "イヤホン", hiragana: "いやほん" },
    Question { japanese: "マイク", hiragana: "まいく" },
    Question { japanese: "カメラ", hiragana: "かめら" },

    // --- Rust・プログラミング特有 (Rust Specifics) ---
    Question { japanese: "構造体", hiragana: "こうぞうたい" },
    Question { japanese: "列挙型", hiragana: "れっきょがた" },
    Question { japanese: "関数", hiragana: "かんすう" },
    Question { japanese: "変数", hiragana: "へんすう" },
    Question { japanese: "定数", hiragana: "ていすう" },
    Question { japanese: "不変", hiragana: "ふへん" },
    Question { japanese: "可変", hiragana: "かへん" },
    Question { japanese: "参照", hiragana: "さんしょう" },
    Question { japanese: "ポインタ", hiragana: "ぽいんた" },
    Question { japanese: "スライス", hiragana: "すらいす" },
    Question { japanese: "ベクタ", hiragana: "べくた" },
    Question { japanese: "文字列", hiragana: "もじれつ" },
    Question { japanese: "整数", hiragana: "せいすう" },
    Question { japanese: "浮動小数点", hiragana: "ふどうしょうすうてん" },
    Question { japanese: "論理値", hiragana: "ろんりち" },
    Question { japanese: "タプル", hiragana: "たぷる" },
    Question { japanese: "配列", hiragana: "はいれつ" },
    Question { japanese: "イテレータ", hiragana: "いてれーた" },
    Question { japanese: "クロージャ", hiragana: "くろーじゃ" },
    Question { japanese: "マクロ", hiragana: "まくろ" },
    Question { japanese: "モジュール", hiragana: "もじゅーる" },
    Question { japanese: "クレート", hiragana: "くれーと" },
    Question { japanese: "パッケージ", hiragana: "ぱっけーじ" },
    Question { japanese: "依存関係", hiragana: "いぞんかんけい" },
    Question { japanese: "テスト駆動開発", hiragana: "てすとくどうかいはつ" },
    Question { japanese: "並行処理", hiragana: "へいこうしょり" },
    Question { japanese: "非同期処理", hiragana: "ひどうきしょり" },
    Question { japanese: "排他制御", hiragana: "はいたせいぎょ" },
    Question { japanese: "メモリリーク", hiragana: "めもりりーく" },
    Question { japanese: "ヌルポインタ", hiragana: "ぬるぽいんた" }, // Rustにはないけど概念として
    Question { japanese: "スタック", hiragana: "すたっく" },
    Question { japanese: "ヒープ", hiragana: "ひーぷ" },
    Question { japanese: "バイナリ", hiragana: "ばいなり" },
    Question { japanese: "ライブラリ", hiragana: "らいぶらり" },
    Question { japanese: "フレームワーク", hiragana: "ふれーむわーく" },
    Question { japanese: "ターミナル", hiragana: "たーみなる" },
    Question { japanese: "コマンド", hiragana: "こまんど" },

    // --- 短文・会話 (Short Sentences) ---
    Question { japanese: "おはようございます", hiragana: "おはようございます" },
    Question { japanese: "こんにちは", hiragana: "こんにちは" },
    Question { japanese: "こんばんは", hiragana: "こんばんは" },
    Question { japanese: "おやすみなさい", hiragana: "おやすみなさい" },
    Question { japanese: "ありがとうございます", hiragana: "ありがとうございます" },
    Question { japanese: "ごめんなさい", hiragana: "ごめんなさい" },
    Question { japanese: "おめでとう", hiragana: "おめでとう" },
    Question { japanese: "さようなら", hiragana: "さようなら" },
    Question { japanese: "いってきます", hiragana: "いってきます" },
    Question { japanese: "いってらっしゃい", hiragana: "いってらっしゃい" },
    Question { japanese: "ただいま", hiragana: "ただいま" },
    Question { japanese: "おかえりなさい", hiragana: "おかえりなさい" },
    Question { japanese: "いただきます", hiragana: "いただきます" },
    Question { japanese: "ごちそうさまでした", hiragana: "ごちそうさまでした" },
    Question { japanese: "はじめまして", hiragana: "はじめまして" },
    Question { japanese: "お元気ですか", hiragana: "おげんきですか" },
    Question { japanese: "調子はどうですか", hiragana: "ちょうしはどうですか" },
    Question { japanese: "いい天気ですね", hiragana: "いいてんきですね" },
    Question { japanese: "何時ですか", hiragana: "なんじですか" },
    Question { japanese: "お腹が空きました", hiragana: "おなかがすきました" },
    Question { japanese: "喉が渇きました", hiragana: "のどがかわきました" },
    Question { japanese: "眠いです", hiragana: "ねむいです" },
    Question { japanese: "疲れました", hiragana: "つかれました" },
    Question { japanese: "頑張りましょう", hiragana: "がんばりましょう" },
    Question { japanese: "楽しみですね", hiragana: "たのしみですね" },
    Question { japanese: "なるほど", hiragana: "なるほど" },
    Question { japanese: "確かに", hiragana: "たしかに" },
    Question { japanese: "その通りです", hiragana: "そのとおりです" },
    Question { japanese: "分かりました", hiragana: "わかりました" },
    Question { japanese: "知りませんでした", hiragana: "しりませんでした" },
    Question { japanese: "教えてください", hiragana: "おしえてください" },
    Question { japanese: "助けてください", hiragana: "たすけてください" },
    Question { japanese: "待ってください", hiragana: "まってください" },
    Question { japanese: "急いでください", hiragana: "いそいでください" },
    Question { japanese: "気をつけて", hiragana: "きをつけて" },
    Question { japanese: "また会いましょう", hiragana: "またあいましょう" },
    Question { japanese: "良い一日を", hiragana: "よいいちにちを" },
    Question { japanese: "お疲れ様でした", hiragana: "おつかれさまでした" },
    Question { japanese: "失礼します", hiragana: "しつれいします" },
    Question { japanese: "もしもし", hiragana: "もしもし" },
    Question { japanese: "準備完了", hiragana: "じゅんびかんりょう" },
    Question { japanese: "出発進行", hiragana: "しゅっぱつしんこう" },
    Question { japanese: "安全第一", hiragana: "あんぜんだいいち" },
    Question { japanese: "整理整頓", hiragana: "せいりせいとん" },
    Question { japanese: "火の用心", hiragana: "ひのようじん" },

    // --- MARK:基礎理論・アルゴリズム (Theory & Algorithms) ---
    Question { japanese: "二進数", hiragana: "にしんすう" },
    Question { japanese: "十六進数", hiragana: "じゅうろくしんすう" },
    Question { japanese: "論理演算", hiragana: "ろんりえんざん" },
    Question { japanese: "フローチャート", hiragana: "ふろーちゃーと" },
    Question { japanese: "探索アルゴリズム", hiragana: "たんさくあるごりずむ" },
    Question { japanese: "整列アルゴリズム", hiragana: "せいれつあるごりずむ" },
    Question { japanese: "二分探索", hiragana: "にぶんたんさく" },
    Question { japanese: "ハッシュ法", hiragana: "はっしゅほう" },
    Question { japanese: "キュー", hiragana: "きゅー" },
    Question { japanese: "スタック", hiragana: "すたっく" },
    Question { japanese: "木構造", hiragana: "きこうぞう" },

    // --- ハードウェア・システム (Hardware & Systems) ---
    Question { japanese: "中央処理装置", hiragana: "ちゅうおうしょりそうち" }, // CPU
    Question { japanese: "主記憶装置", hiragana: "しゅきおくそうち" }, // メモリ
    Question { japanese: "補助記憶装置", hiragana: "ほじょきおくそうち" }, // ストレージ
    Question { japanese: "キャッシュメモリ", hiragana: "きゃっしゅめもり" },
    Question { japanese: "クロック周波数", hiragana: "くろっくしゅうはすう" },
    Question { japanese: "バス", hiragana: "ばす" },
    Question { japanese: "インタフェース", hiragana: "いんたふぇーす" },
    Question { japanese: "デバイスドライバ", hiragana: "でばいすどらいば" },
    Question { japanese: "プラグアンドプレイ", hiragana: "ぷらぐあんどぷれい" },
    Question { japanese: "ソリッドステートドライブ", hiragana: "そりっどすてーとどらいぶ" }, // SSD

    // --- ソフトウェア・OS (Software & OS) ---
    Question { japanese: "オペレーティングシステム", hiragana: "おぺれーてぃんぐしすてむ" },
    Question { japanese: "ミドルウェア", hiragana: "みどるうぇあ" },
    Question { japanese: "ファイルシステム", hiragana: "ふぁいるしすてむ" },
    Question { japanese: "ディレクトリ", hiragana: "でぃれくとり" },
    Question { japanese: "バックアップ", hiragana: "ばっくあっぷ" },
    Question { japanese: "アーカイブ", hiragana: "あーかいぶ" },
    Question { japanese: "オープンソースソフトウェア", hiragana: "おーぷんそーすそふとうぇあ" }, // OSS
    Question { japanese: "ライセンス", hiragana: "らいせんす" },
    Question { japanese: "バッチ処理", hiragana: "ばっちしょり" },
    Question { japanese: "リアルタイム処理", hiragana: "りあるたいむしょり" },

    // --- データベース (Database) ---
    Question { japanese: "関係データベース", hiragana: "かんけいでーたべーす" }, // RDB
    Question { japanese: "主キー", hiragana: "しゅきー" },
    Question { japanese: "外部キー", hiragana: "がいぶきー" },
    Question { japanese: "正規化", hiragana: "せいきか" },
    Question { japanese: "トランザクション", hiragana: "とらんざくしょん" },
    Question { japanese: "排他制御", hiragana: "はいたせいぎょ" },
    Question { japanese: "デッドロック", hiragana: "でっどろっく" },
    Question { japanese: "データウェアハウス", hiragana: "でーたうぇあはうす" },
    Question { japanese: "ビッグデータ", hiragana: "びっぐでーた" },
    Question { japanese: "データマイニング", hiragana: "でーたまいにんぐ" },

    // --- ネットワーク (Network) ---
    Question { japanese: "プロトコル", hiragana: "ぷろとこる" },
    Question { japanese: "ローカルエリアネットワーク", hiragana: "ろーかるえりあねっとわーく" }, // LAN
    Question { japanese: "アイピーアドレス", hiragana: "あいぴーあどれす" }, // IPアドレス
    Question { japanese: "ドメイン名", hiragana: "どめいんめい" },
    Question { japanese: "ドメインネームシステム", hiragana: "どめいんねーむしすてむ" }, // DNS
    Question { japanese: "ルータ", hiragana: "るーた" },
    Question { japanese: "パケット", hiragana: "ぱけっと" },
    Question { japanese: "ファイアウォール", hiragana: "ふぁいあうぉーる" },
    Question { japanese: "無線ラン", hiragana: "むせんらん" }, // 無線LAN
    Question { japanese: "ブロードバンド", hiragana: "ぶろーどばんど" },

    // --- セキュリティ (Security) ---
    Question { japanese: "情報セキュリティ", hiragana: "じょうほうせきゅりてぃ" },
    Question { japanese: "機密性", hiragana: "きみつせい" },
    Question { japanese: "完全性", hiragana: "かんぜんせい" },
    Question { japanese: "可用性", hiragana: "かようせい" },
    Question { japanese: "マルウェア", hiragana: "まるうぇあ" },
    Question { japanese: "コンピュータウイルス", hiragana: "こんぴゅーたういるす" },
    Question { japanese: "フィッシング詐欺", hiragana: "ふぃっしんぐさぎ" },
    Question { japanese: "ソーシャルエンジニアリング", hiragana: "そーしゃるえんじにありんぐ" },
    Question { japanese: "暗号化", hiragana: "あんごうか" },
    Question { japanese: "デジタル署名", hiragana: "でじたるしょめい" },
    Question { japanese: "認証", hiragana: "にんしょう" },
    Question { japanese: "バイオメトリクス", hiragana: "ばいおめとりくす" },
    Question { japanese: "ワンタイムパスワード", hiragana: "わんたいむぱすわーど" },

    // --- 経営・マネジメント (Management & Strategy) ---
    Question { japanese: "コンプライアンス", hiragana: "こんぷらいあんす" },
    Question { japanese: "コーポレートガバナンス", hiragana: "こーぽれーとがばなんす" },
    Question { japanese: "ケーピーアイ", hiragana: "けーぴーあい" }, // KPI
    Question { japanese: "ピーディーシーエー", hiragana: "ぴーでぃーしーえー" }, // PDCA
    Question { japanese: "エスダブリューオーティー分析", hiragana: "えすだぶりゅーおーてぃーぶんせき" }, // SWOT分析
    Question { japanese: "サプライチェーンマネジメント", hiragana: "さぷらいちぇーんまねじめんと" }, // SCM
    Question { japanese: "カスタマーリレーションシップ", hiragana: "かすたまーりれーしょんしっぷ" }, // CRM
    Question { japanese: "ビジネスプロセスアウトソーシング", hiragana: "びじねすぷろせすあうとそーしんぐ" }, // BPO
    Question { japanese: "サービスレベルアグリーメント", hiragana: "さーびすれべるあぐりーめんと" }, // SLA
    Question { japanese: "プロジェクトマネジメント", hiragana: "ぷろじぇくとまねじめんと" },

    // --- MARK:システム開発・テスト (System Development & Testing) ---
    Question { japanese: "要件定義", hiragana: "ようけんていぎ" },
    Question { japanese: "外部設計", hiragana: "がいぶせっけい" },
    Question { japanese: "内部設計", hiragana: "ないぶせっけい" },
    Question { japanese: "プログラム設計", hiragana: "ぷろぐらむせっけい" },
    Question { japanese: "単体テスト", hiragana: "たんたいてすと" },
    Question { japanese: "結合テスト", hiragana: "けつごうてすと" },
    Question { japanese: "システムテスト", hiragana: "しすてむてすと" },
    Question { japanese: "運用テスト", hiragana: "うんようてすと" },
    Question { japanese: "ホワイトボックステスト", hiragana: "ほわいとぼっくすてすと" },
    Question { japanese: "ブラックボックステスト", hiragana: "ぶらっくぼっくすてすと" },
    Question { japanese: "ウォーターフォールモデル", hiragana: "うぉーたーふぉーるもでる" },
    Question { japanese: "アジャイル開発", hiragana: "あじゃいるかいはつ" },
    Question { japanese: "プロトタイピング", hiragana: "ぷろとたいぴんぐ" },
    Question { japanese: "スパイラルモデル", hiragana: "すぱいらるもでる" },
    Question { japanese: "デブオプス", hiragana: "でぶおぷす" }, // DevOps
    Question { japanese: "リファクタリング", hiragana: "りふぁくたりんぐ" },
    Question { japanese: "バージョン管理", hiragana: "ばーじょんかんり" },
    Question { japanese: "回帰テスト", hiragana: "かいきてすと" }, // リグレッションテスト

    // --- プロジェクトマネジメント・図表 (PM & Charts) ---
    Question { japanese: "プロジェクト憲章", hiragana: "ぷろじぇくとけんしょう" },
    Question { japanese: "ワークブレークダウンストラクチャ", hiragana: "わーくぶれーくだうんすとらくちゃ" }, // WBS
    Question { japanese: "ガントチャート", hiragana: "がんとちゃーと" },
    Question { japanese: "アローダイアグラム", hiragana: "あろーだいあぐらむ" },
    Question { japanese: "クリティカルパス", hiragana: "くりてぃかるぱす" },
    Question { japanese: "マイルストーン", hiragana: "まいるすとーん" },
    Question { japanese: "ステークホルダ", hiragana: "すてーくほるだ" },
    Question { japanese: "フィッシュボーンダイアグラム", hiragana: "ふぃっしゅぼーんだいあぐらむ" }, // 特性要因図
    Question { japanese: "パレート図", hiragana: "ぱれーとず" },
    Question { japanese: "ヒストグラム", hiragana: "ひすとぐらむ" },
    Question { japanese: "散布図", hiragana: "さんぷず" },
    Question { japanese: "管理図", hiragana: "かんりず" },
    Question { japanese: "ブレーンストーミング", hiragana: "ぶれーんすとーみんぐ" },

    // --- サービスマネジメント (Service Management) ---
    Question { japanese: "アイティル", hiragana: "あいてぃる" }, // ITIL
    Question { japanese: "サービスデスク", hiragana: "さーびすですく" },
    Question { japanese: "インシデント管理", hiragana: "いんしでんとかんり" },
    Question { japanese: "問題管理", hiragana: "もんだいかんり" },
    Question { japanese: "変更管理", hiragana: "へんこうかんり" },
    Question { japanese: "リリース管理", hiragana: "りりーすかんり" },
    Question { japanese: "構成管理", hiragana: "こうせいかんり" },
    Question { japanese: "可用性管理", hiragana: "かようせいかんり" },
    Question { japanese: "キャパシティ管理", hiragana: "きゃぱしてぃかんり" },
    Question { japanese: "事業継続計画", hiragana: "じぎょうけいぞくけいかく" }, // BCP

    // --- 法務・コンプライアンス (Legal & Compliance) ---
    Question { japanese: "知的財産権", hiragana: "ちてきざいさんけん" },
    Question { japanese: "著作権", hiragana: "ちょさくけん" },
    Question { japanese: "産業財産権", hiragana: "さんぎょうざいさんけん" },
    Question { japanese: "特許権", hiragana: "とっきょけん" },
    Question { japanese: "実用新案権", hiragana: "じつようしんあんけん" },
    Question { japanese: "意匠権", hiragana: "いしょうけん" },
    Question { japanese: "商標権", hiragana: "しょうひょうけん" },
    Question { japanese: "トレードシークレット", hiragana: "とれーどしーくれっと" }, // 営業秘密
    Question { japanese: "個人情報保護法", hiragana: "こじんじょうほうほごほう" },
    Question { japanese: "マイナンバー法", hiragana: "まいなんばーほう" },
    Question { japanese: "不正アクセス禁止法", hiragana: "ふせいあくせすきんしほう" },
    Question { japanese: "刑法", hiragana: "けいほう" }, // 電子計算機損壊等業務妨害罪など
    Question { japanese: "労働基準法", hiragana: "ろうどうきじゅんほう" },
    Question { japanese: "派遣法", hiragana: "はけんほう" },
    Question { japanese: "製造物責任法", hiragana: "せいぞうぶつせきにんほう" }, // PL法
    Question { japanese: "特定商取引法", hiragana: "とくていしょうとりひきほう" },
    Question { japanese: "シュリンクラップ契約", hiragana: "しゅりんくらっぷけいやく" },
    Question { japanese: "ボリュームライセンス", hiragana: "ぼりゅーむらいせんす" },
    Question { japanese: "サイトライセンス", hiragana: "さいとらいせんす" },

    // --- 企業活動・会計 (Business & Accounting) ---
    Question { japanese: "企業の社会的責任", hiragana: "きぎょうのしゃかいてきせきにん" }, // CSR
    Question { japanese: "グリーンアイティー", hiragana: "ぐりーんあいてぃー" },
    Question { japanese: "職能別組織", hiragana: "しょくのうべつそしき" },
    Question { japanese: "事業部制組織", hiragana: "じぎょうぶせいそしき" },
    Question { japanese: "マトリックス組織", hiragana: "まとりっくすそしき" },
    Question { japanese: "プロジェクト組織", hiragana: "ぷろじぇくとそしき" },
    Question { japanese: "シーイーオー", hiragana: "しーいーおー" }, // CEO
    Question { japanese: "シーアイオー", hiragana: "しーあいおー" }, // CIO
    Question { japanese: "財務諸表", hiragana: "ざいむしょひょう" },
    Question { japanese: "貸借対照表", hiragana: "たいしゃくたいしょうひょう" }, // B/S
    Question { japanese: "損益計算書", hiragana: "そんえきけいさんしょ" }, // P/L
    Question { japanese: "キャッシュフロー計算書", hiragana: "きゃっしゅふろーけいさんしょ" },
    Question { japanese: "損益分岐点", hiragana: "そんえきぶんきてん" },
    Question { japanese: "自己資本比率", hiragana: "じこしほんひりつ" },
    Question { japanese: "減価償却", hiragana: "げんかしょうきゃく" },
    Question { japanese: "流動資産", hiragana: "りゅうどうしさん" },
    Question { japanese: "固定資産", hiragana: "こていしさん" },
    Question { japanese: "負債", hiragana: "ふさい" },
    Question { japanese: "純資産", hiragana: "じゅんしさん" },
    Question { japanese: "売上総利益", hiragana: "うりあげそうりえき" },
    Question { japanese: "営業利益", hiragana: "えいぎょうりえき" },
    Question { japanese: "経常利益", hiragana: "けいじょうりえき" },

    // --- 先端技術・トレンド (New Tech & Trends) ---
    Question { japanese: "人工知能", hiragana: "じんこうちのう" },
    Question { japanese: "ディープラーニング", hiragana: "でぃーぷらーにんぐ" }, // 深層学習
    Question { japanese: "ニューラルネットワーク", hiragana: "にゅーらるねっとわーく" },
    Question { japanese: "モノのインターネット", hiragana: "もののいんたーねっと" }, // IoT
    Question { japanese: "デジタルトランスフォーメーション", hiragana: "でじたるとらんすふぉーめーしょん" }, // DX
    Question { japanese: "フィンテック", hiragana: "ふぃんてっく" },
    Question { japanese: "仮想現実", hiragana: "かそうげんじつ" }, // VR
    Question { japanese: "拡張現実", hiragana: "かくちょうげんじつ" }, // AR
    Question { japanese: "ドローン", hiragana: "どろーん" },
    Question { japanese: "エッジコンピューティング", hiragana: "えっじこんぴゅーてぃんぐ" },
    Question { japanese: "量子コンピュータ", hiragana: "りょうしこんぴゅーた" },
    Question { japanese: "スマートシティ", hiragana: "すまーとしてぃ" },
    Question { japanese: "テレワーク", hiragana: "てれわーく" },
    Question { japanese: "クラウドファンディング", hiragana: "くらうどふぁんでぃんぐ" },
    Question { japanese: "シェアリングエコノミー", hiragana: "しぇありんぐえこのみー" },
    Question { japanese: "サブスクリプション", hiragana: "さぶすくりぷしょん" },
   
    // --- MARK:セキュリティ・攻撃手法 (Security & Attacks) ---
    Question { japanese: "ランサムウェア", hiragana: "らんさむうぇあ" },
    Question { japanese: "トロイの木馬", hiragana: "とろいのもくば" },
    Question { japanese: "スパイウェア", hiragana: "すぱいうぇあ" },
    Question { japanese: "キーロガー", hiragana: "きーろがー" },
    Question { japanese: "ボットネット", hiragana: "ぼっとねっと" },
    Question { japanese: "ゼロデイ攻撃", hiragana: "ぜろでいこうげき" },
    Question { japanese: "総当たり攻撃", hiragana: "そうあたりこうげき" },
    Question { japanese: "辞書攻撃", hiragana: "じしょこうげき" },
    Question { japanese: "クロスサイトスクリプティング", hiragana: "くろすさいとすくりぷてぃんぐ" }, // XSS
    Question { japanese: "エスキューエルインジェクション", hiragana: "えすきゅーえるいんじぇくしょん" }, // SQLi
    Question { japanese: "セッションハイジャック", hiragana: "せっしょんはいじゃっく" },
    Question { japanese: "バッファオーバーフロー", hiragana: "ばっふぁおーばーふろー" },
    Question { japanese: "バックドア", hiragana: "ばっくどあ" },
    Question { japanese: "ハニーポット", hiragana: "はにーぽっと" },
    Question { japanese: "デジタルフォレンジック", hiragana: "でじたるふぉれんじっく" },
    Question { japanese: "公開鍵基盤", hiragana: "こうかいかぎきばん" }, // PKI
    Question { japanese: "認証局", hiragana: "にんしょうきょく" }, // CA
    Question { japanese: "仮想私設網", hiragana: "かそうしせつもう" }, // VPN
    Question { japanese: "侵入検知システム", hiragana: "しんにゅうけんちしすてむ" }, // IDS
    Question { japanese: "統一脅威管理", hiragana: "とういつきょういかんり" }, // UTM
    Question { japanese: "二要素認証", hiragana: "にようそにんしょう" },
    Question { japanese: "キャプチャ", hiragana: "きゃぷちゃ" }, // CAPTCHA

    // --- ネットワーク・通信 (Network & Communication) ---
    Question { japanese: "伝送制御プロトコル", hiragana: "でんそうせいぎょぷろとこる" }, // TCP
    Question { japanese: "ユーザデータグラムプロトコル", hiragana: "ゆーざでーたぐらむぷろとこる" }, // UDP
    Question { japanese: "ファイル転送プロトコル", hiragana: "ふぁいるてんそうぷろとこる" }, // FTP
    Question { japanese: "ハイパーテキスト転送プロトコル", hiragana: "はいぱーてきすとてんそうぷろとこる" }, // HTTP
    Question { japanese: "簡易メール転送プロトコル", hiragana: "かんいめーるてんそうぷろとこる" }, // SMTP
    Question { japanese: "動的ホスト構成プロトコル", hiragana: "どうてきほすとこうせいぷろとこる" }, // DHCP
    Question { japanese: "ネットワークアドレス変換", hiragana: "ねっとわーくあどれすへんかん" }, // NAT
    Question { japanese: "サブネットマスク", hiragana: "さぶねっとますく" },
    Question { japanese: "デフォルトゲートウェイ", hiragana: "でふぉるとげーとうぇい" },
    Question { japanese: "マックアドレス", hiragana: "まっくあどれす" }, // MAC Address
    Question { japanese: "グローバルＩＰアドレス", hiragana: "ぐろーばるあいぴーあどれす" },
    Question { japanese: "プライベートＩＰアドレス", hiragana: "ぷらいべーとあいぴーあどれす" },
    Question { japanese: "仮想移動体通信事業者", hiragana: "かそういどうたいつうしんじぎょうしゃ" }, // MVNO
    Question { japanese: "テザリング", hiragana: "てざりんぐ" },
    Question { japanese: "近距離無線通信", hiragana: "きんきょりむせんつうしん" }, // NFC
    Question { japanese: "ビーコン", hiragana: "びーこん" },
    Question { japanese: "光ファイバ", hiragana: "ひかりふぁいば" },
    Question { japanese: "パケット交換", hiragana: "ぱけっとこうかん" },

    // --- システム構成・信頼性 (System & Reliability) ---
    Question { japanese: "レイド", hiragana: "れいど" }, // RAID
    Question { japanese: "ミラーリング", hiragana: "みらーりんぐ" },
    Question { japanese: "ストライピング", hiragana: "すとらいぴんぐ" },
    Question { japanese: "デュアルシステム", hiragana: "でゅあるしすてむ" },
    Question { japanese: "デュプレックスシステム", hiragana: "でゅぷれっくすしすてむ" },
    Question { japanese: "平均故障間隔", hiragana: "へいきんこしょうかんかく" }, // MTBF
    Question { japanese: "平均修復時間", hiragana: "へいきんしゅうふくじかん" }, // MTTR
    Question { japanese: "稼働率", hiragana: "かどうりつ" },
    Question { japanese: "バスタブ曲線", hiragana: "ばすたぶきょくせん" },
    Question { japanese: "フォールトトレラント", hiragana: "ふぉーるととれらんと" },
    Question { japanese: "フェイルセーフ", hiragana: "ふぇいるせーふ" },
    Question { japanese: "フェイルソフト", hiragana: "ふぇいるそふと" },
    Question { japanese: "フールプルーフ", hiragana: "ふーるぷるーふ" },
    Question { japanese: "ユニバーサルデザイン", hiragana: "ゆにばーさるでざいん" },
    Question { japanese: "アクセシビリティ", hiragana: "あくせしびりてぃ" },

    // --- 開発手法・オブジェクト指向 (Dev Methods & OOP) ---
    Question { japanese: "オブジェクト指向", hiragana: "おぶじぇくとしこう" },
    Question { japanese: "カプセル化", hiragana: "かぷせるか" },
    Question { japanese: "継承", hiragana: "けいしょう" }, // インヘリタンス
    Question { japanese: "ポリモーフィズム", hiragana: "ぽりもーふぃずむ" }, // 多態性
    Question { japanese: "クラス", hiragana: "くらす" },
    Question { japanese: "インスタンス", hiragana: "いんすたんす" },
    Question { japanese: "ユニファイドモデリング言語", hiragana: "ゆにふぁいどもでりんぐげんご" }, // UML
    Question { japanese: "ユースケース図", hiragana: "ゆーすけーすず" },
    Question { japanese: "シーケンス図", hiragana: "しーけんすず" },
    Question { japanese: "クラス図", hiragana: "くらすず" },
    Question { japanese: "エクストリームプログラミング", hiragana: "えくすとりーむぷろぐらみんぐ" }, // XP
    Question { japanese: "スクラム", hiragana: "すくらむ" },
    Question { japanese: "ペアプログラミング", hiragana: "ぺあぷろぐらみんぐ" },
    Question { japanese: "コードレビュー", hiragana: "こーどれびゅー" },
    Question { japanese: "継続的インテグレーション", hiragana: "けいぞくてきいんてぐれーしょん" }, // CI

    // --- ビジネス戦略・マーケティング (Business & Marketing) ---
    Question { japanese: "電子商取引", hiragana: "でんししょうとりひき" }, // EC
    Question { japanese: "企業間取引", hiragana: "きぎょうかんとりひき" }, // B2B
    Question { japanese: "消費者間取引", hiragana: "しょうひしゃかんとりひき" }, // C2C
    Question { japanese: "オンラインツーオフライン", hiragana: "おんらいんつーおふらいん" }, // O2O
    Question { japanese: "ロングテール", hiragana: "ろんぐてーる" },
    Question { japanese: "検索エンジン最適化", hiragana: "けんさくえんじんさいてきか" }, // SEO
    Question { japanese: "アフィリエイト", hiragana: "あふぃりえいと" },
    Question { japanese: "クラウドソーシング", hiragana: "くらうどそーしんぐ" },
    Question { japanese: "ギグエコノミー", hiragana: "ぎぐえこのみー" },
    Question { japanese: "ブルーオーシャン戦略", hiragana: "ぶるーおーしゃんせんりゃく" },
    Question { japanese: "プロダクトライフサイクル", hiragana: "ぷろだくとらいふさいくる" },
    Question { japanese: "ニッチ戦略", hiragana: "にっちせんりゃく" },
    Question { japanese: "セグメンテーション", hiragana: "せぐめんてーしょん" },
    Question { japanese: "ターゲティング", hiragana: "たーげてぃんぐ" },
    Question { japanese: "ポジショニング", hiragana: "ぽじしょにんぐ" },
    Question { japanese: "マーチャンダイジング", hiragana: "まーちゃん代じんぐ" },
    Question { japanese: "ロジスティクス", hiragana: "ろじすてぃくす" },
    Question { japanese: "ジャストインタイム", hiragana: "じゃすといんたいむ" },
    Question { japanese: "コアコンピタンス", hiragana: "こあこんぴたんす" },
    Question { japanese: "ベンチマーキング", hiragana: "べんちまーきんぐ" },
    Question { japanese: "エムアンドエー", hiragana: "えむあんどえー" }, // M&A
    Question { japanese: "イニシャルパブリックオファリング", hiragana: "いにしゃるぱぶりっくおふぁりんぐ" }, // IPO

    // --- 会計・財務 (Accounting & Finance) ---
    Question { japanese: "自己資本利益率", hiragana: "じこしほんりえきりつ" }, // ROE
    Question { japanese: "投資対効果", hiragana: "とうしたいこうか" }, // ROI
    Question { japanese: "流動比率", hiragana: "りゅうどうひりつ" },
    Question { japanese: "当座比率", hiragana: "とうざひりつ" },
    Question { japanese: "固定費", hiragana: "こていひ" },
    Question { japanese: "変動費", hiragana: "へんどうひ" },
    Question { japanese: "損益分岐点売上高", hiragana: "そんえきぶんきてんうりあげだか" },
    Question { japanese: "減価償却費", hiragana: "げんかしょうきゃくひ" },
    Question { japanese: "棚卸資産", hiragana: "たなおろししさん" },
    Question { japanese: "売掛金", hiragana: "うりかけきん" },
    Question { japanese: "買掛金", hiragana: "かいかけきん" },

    // --- 法務・ガバナンス (Legal & Governance) ---
    Question { japanese: "説明責任", hiragana: "せつめいせきにん" }, // Accountability
    Question { japanese: "ディスクロージャー", hiragana: "でぃすくろーじゃー" },
    Question { japanese: "機密保持契約", hiragana: "きみつほじけいやく" }, // NDA
    Question { japanese: "サービスレベル合意書", hiragana: "さーびすれべるごういしょ" }, // SLA
    Question { japanese: "内部統制", hiragana: "ないぶとうせい" },
    Question { japanese: "公益通報者保護法", hiragana: "こうえきつうほうしゃほごほう" },
    Question { japanese: "製造物責任法", hiragana: "せいぞうぶつせきにんほう" }, // PL法
    Question { japanese: "特定商取引法", hiragana: "とくていしょうとりひきほう" },

    // --- DX・新技術・その他 (DX & Emerging Tech) ---
    Question { japanese: "ロボティックプロセスオートメーション", hiragana: "ろぼてぃっくぷろせすおーとめーしょん" }, // RPA
    Question { japanese: "チャットボット", hiragana: "ちゃっとぼっと" },
    Question { japanese: "スマートコントラクト", hiragana: "すまーとこんとらくと" },
    Question { japanese: "ノンファンジブルトークン", hiragana: "のんふぁんじぶるとーくん" }, // NFT
    Question { japanese: "メタバース", hiragana: "めたばーす" },
    Question { japanese: "デジタルツイン", hiragana: "でじたるついん" },
    Question { japanese: "シンギュラリティ", hiragana: "しんぎゅらりてぃ" }, // 技術的特異点
    Question { japanese: "エッジコンピューティング", hiragana: "えっじこんぴゅーてぃんぐ" },
    Question { japanese: "ウェアラブルデバイス", hiragana: "うぇあらぶるでばいす" },
    Question { japanese: "ヘッドマウントディスプレイ", hiragana: "へっどまうんとでぃすぷれい" },
    Question { japanese: "スマートグリッド", hiragana: "すまーとぐりっど" },
    Question { japanese: "コネクテッドカー", hiragana: "こねくてっどかー" },
    Question { japanese: "自動運転", hiragana: "じどううんてん" },
    Question { japanese: "ドローン配送", hiragana: "どろーんはいそう" },
    Question { japanese: "スリーディープリンタ", hiragana: "すりーでぃーぷりんた" }, // 3Dプリンタ
    Question { japanese: "ソサエティ５．０", hiragana: "そさえてぃごてんぜろ" }, // Society 5.0
    Question { japanese: "インダストリー４．０", hiragana: "いんだすとりーよんてんぜろ" }, // Industry 4.0
    Question { japanese: "プラットフォーマー", hiragana: "ぷらっとふぉーまー" },
    Question { japanese: "ガーファ", hiragana: "がーふぁ" }, // GAFA
    Question { japanese: "デファクトスタンダード", hiragana: "でふぁくとすたんだーど" },
];

