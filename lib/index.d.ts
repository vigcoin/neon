export declare class Scalar {
  static setupRandom(value: number): void;
  static checkBuffer(scalar: Buffer): boolean;
  static checkHexString(scalar: string): boolean;
  static random(): Buffer;
  static toHash(scalar: Buffer): Buffer;
  static fromHash(hash: Buffer): Buffer;
  static toPoint(hash: Buffer): Buffer;
}

export declare class Key {
  static generateKeys(): { public: Buffer, secret: Buffer };
  static check(hash: Buffer): boolean;
  static secretToPublic(secretKey: Buffer): Buffer;
  static derivate(publicKey: Buffer, secretKey: Buffer): Buffer;
  static derivePublicKey(derivation: Buffer, secretKey: Buffer, index: number): Buffer;
  static underivePublicKey(derivation: Buffer, publicKey: Buffer, index: number): Buffer;
  static generateImage(publicKey: Buffer, secretKey: Buffer): Buffer;
}

export declare class Signature {
  static generate(prefixHash: Buffer, publicKey: Buffer, secretKey: Buffer): Buffer;
  static check(prefixHash: Buffer, publicKey: Buffer, signature: Buffer): Buffer;
  static generateRing(prefixHash: Buffer, image: Buffer, pubsv: Buffer[], pubsCount: number, secretKey: Buffer, secretKeyIndex
    : number): Buffer;
  static checkRing(prefixHash: Buffer, image: Buffer, pubsv: Buffer[], pubsCount: number, signatures: Buffer): Buffer;
}

export declare class Hash {
  static fast(buffer: Buffer): string;
  static slow(buffer: Buffer, variant: number): string;
  static check(buffer: Buffer, difficulty: number): boolean;
  static from(buffer: Buffer): Buffer;
}

export declare class Amount {
  static getPenalized(amount: number, medianSize: number, currentBlockSize: number): number;
}

export declare class Difficulty {
  constructor(
    target: number,
    cut: number,
    lag: number,
    window: number
  );
  public next(timestamps: number[], difficulties: number[]): string;
}

export declare class Wallet {
  constructor(
    filename?: string,
    password?: string
  );
  public create(prefix: number): { spend: string, view: string, address: string };
  public setPrivateKeys(spend: string, view: string): void;
  public getPrivateKeys(): { spend: string, view: string };
  public save(
    filename: string,
    password: string
  ): void;
  public toAddress(prefix: number): string;
}
