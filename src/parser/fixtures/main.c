__asm__(".global _start\n"
        "_start:\n"
        "    adr     x0, x\n" // x のアドレスを、PC 相対の adr 命令で直接取得
        "    ldr     w0, [x0]\n" // x の値 (32ビット, 例: 23) を x0 にロード（w0 で読み込む）
        "    mov     x8, #93\n" // exit システムコール番号 93 を x8 にセット
        "    svc     #0\n" // システムコールを発行して終了
);
