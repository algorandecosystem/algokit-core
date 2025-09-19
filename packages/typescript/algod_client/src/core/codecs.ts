// Temporary copy of codec helpers from utils-ts until the shared core moves into the monorepo.
// These keep the generated clients self-contained and ready for future DTO <-> domain mappers.

export abstract class Codec<T, TEncoded = T> {
  public abstract defaultValue(): TEncoded;

  protected toEncoded(value: T): TEncoded {
    return value as unknown as TEncoded;
  }

  protected fromEncoded(value: TEncoded): T {
    return value as unknown as T;
  }

  protected isDefaultValue(value: T): boolean {
    return this.toEncoded(value) === this.defaultValue();
  }

  public encode(value?: T): TEncoded | undefined {
    return value !== undefined && !this.isDefaultValue(value) ? this.toEncoded(value) : undefined;
  }

  public decode(value: TEncoded | undefined): T {
    return this.fromEncoded(value ?? this.defaultValue());
  }

  public decodeOptional(value: TEncoded | undefined): T | undefined {
    if (value === undefined) {
      return undefined;
    }
    return this.fromEncoded(value);
  }
}

export class NumberCodec extends Codec<number> {
  public defaultValue(): number {
    return 0;
  }
}

export class BigIntCodec extends Codec<bigint, number | bigint> {
  public defaultValue(): bigint {
    return 0n;
  }

  protected fromEncoded(value: number | bigint): bigint {
    return typeof value === "bigint" ? value : BigInt(value);
  }
}

export class StringCodec extends Codec<string> {
  public defaultValue(): string {
    return "";
  }
}

export class BytesCodec extends Codec<Uint8Array> {
  public defaultValue(): Uint8Array {
    return new Uint8Array();
  }

  protected isDefaultValue(value: Uint8Array): boolean {
    return value.byteLength === 0;
  }
}

export class BooleanCodec extends Codec<boolean> {
  public defaultValue(): boolean {
    return false;
  }
}

export class OmitEmptyObjectCodec<T extends object> extends Codec<T, T | undefined> {
  public defaultValue(): T | undefined {
    return undefined;
  }

  protected isDefaultValue(value: T): boolean {
    return Object.values(value).filter((x) => x !== undefined).length === 0;
  }
}

export const numberCodec = new NumberCodec();
export const bigIntCodec = new BigIntCodec();
export const stringCodec = new StringCodec();
export const bytesCodec = new BytesCodec();
export const booleanCodec = new BooleanCodec();
