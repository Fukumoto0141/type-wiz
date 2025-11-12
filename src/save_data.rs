// ============================================
// src/save_data.rs
// セーブデータの構造と読み書きロジック
// ============================================

use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

const SAVE_FILE_NAME: &str = "save_data.json";

/// プレイヤーの進行状況データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub level: u32,
    pub current_xp: u32,
    pub total_typed_chars: u32,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            level: 1,
            current_xp: 0,
            total_typed_chars: 0,
        }
    }
}

impl PlayerData {
    /// 次のレベルまでに必要な経験値を計算する
    /// 計算式: レベル * 100 (例: Lv1->100, Lv2->200...)
    pub fn required_xp_for_next_level(&self) -> u32 {
        self.level * 100
    }

    /// 経験値を加算し、レベルアップ判定を行う
    /// 戻り値: レベルアップした場合は true
    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.current_xp += amount;
        self.total_typed_chars += amount;

        let mut leveled_up = false;
        // 必要経験値を超えている間、レベルを上げ続ける
        while self.current_xp >= self.required_xp_for_next_level() {
            self.current_xp -= self.required_xp_for_next_level();
            self.level += 1;
            leveled_up = true;
        }
        leveled_up
    }

    /// データをファイルに保存する (JSON)
    pub fn save(&self) {
        // `serde_json::to_string_pretty` で整形されたJSON文字列を作成
        if let Ok(json) = serde_json::to_string_pretty(self) {
            // ファイル書き込み (エラーハンドリングは簡略化のため無視していますが、実運用ではログ出力などが望ましい)
            let _ = fs::write(SAVE_FILE_NAME, json);
        }
    }

    /// ファイルからデータを読み込む
    /// ファイルがない、または壊れている場合はデフォルト値を返す
    pub fn load() -> Self {
        if !Path::new(SAVE_FILE_NAME).exists() {
            return Self::default();
        }

        if let Ok(file) = File::open(SAVE_FILE_NAME) {
            let reader = BufReader::new(file);
            if let Ok(data) = serde_json::from_reader(reader) {
                return data;
            }
        }

        // 読み込み失敗時はデフォルト
        Self::default()
    }
}