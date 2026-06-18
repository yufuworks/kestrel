import { describe, it, expect } from "vitest";
import { formatBytes, formatUptime } from "./utils";

describe("formatBytes", () => {
  // byte値を引数 bytes に取り、bytes / 1024 ^ 3 を小数点1桁で四捨五入し
  // GB を単位とした文字列を生成する。
  it("returns 0.0 GB for zero bytes", () => {
    expect(formatBytes(0)).toBe("0.0 GB");
  });
  it("rounds down at boundary (0.049)", () => {
    expect(formatBytes(52613350)).toBe("0.0 GB");
  });
  it("rounds up at boundary (0.050)", () => {
    expect(formatBytes(53687092)).toBe("0.1 GB");
  });
  it("rounds down 1.24 to 1.2", () => {
    expect(formatBytes(1331439862)).toBe("1.2 GB");
  });
});

describe("formatUptime", () => {
  // 秒数を引数 secs に取り、日、時間、分 を計算してフォーマットを生成する。
  it("falls below minute boundary", () => {
    expect(formatUptime(59)).toBe("0日 0時間 0分");
  });
  it("reaches minute boundary", () => {
    expect(formatUptime(60)).toBe("0日 0時間 1分");
  });
  it("falls below hour boundary", () => {
    expect(formatUptime(3599)).toBe("0日 0時間 59分");
  });
  it("reaches hour boundary", () => {
    expect(formatUptime(3600)).toBe("0日 1時間 0分");
  });
  it("falls below day boundary", () => {
    expect(formatUptime(86399)).toBe("0日 23時間 59分");
  });
  it("reaches day boundary", () => {
    expect(formatUptime(86400)).toBe("1日 0時間 0分");
  });
});
