#!/bin/bash
# T-Player One-Command Verification Script

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo "🔍 啟動 T-Player 語法門禁檢查..."
if cargo check &> /dev/null; then
    echo -e "${GREEN}[PASS] 語法檢查通過${NC}"
else
    echo -e "${RED}[FAIL] 語法檢查失敗${NC}"
    exit 1
fi

echo "🧪 執行音訊核心單元測試..."
if cargo test --lib &> /dev/null; then
    echo -e "${GREEN}[PASS] 單元測試通過${NC}"
else
    echo -e "${RED}[FAIL] 單元測試失敗${NC}"
    exit 1
fi

echo -e "
${GREEN}✅ 所有驗證通過！T-Player 已準備就緒。${NC}"
