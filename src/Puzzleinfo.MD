1 BTC challenge with splitted mnemonic code

Address:
bc1qyjwa0tf0en4x09magpuwmt2smpsrlaxwn85lh6 1.00000000tBTC
Zpub:
zpub6qdEDkv51FpxX6g1rpFGckmiL46vV8ccmtEgPAkj3qj8N4ZZHyXDRA9RwpTiFK2Kb8vRaDmSmwgX6rfB4t2K8Ktdq8ExQ6fumKpn2ndJCqL

The 12-word original mnemonic code was split using the Shamir Secret Sharing scheme with 3 out of 5 threshold schemes were used. This means that any three shares are sufficient to restore the original mnemonic code. The goal is to break the Shamir Secret Sharing scheme or break the implementation of software for SSSS. We publish 2 of 3 shares needed to restore the original mnemonics.

Share 1:
session
 
cigar
 
grape
 
merry
 
useful
 
churn
 
fatal
 
thought
 
very
 
any
 
arm
 
unaware
Share 2:
clock
 
fresh
 
security
 
field
 
caution
 
effort
 
gorilla
 
speed
 
plastic
 
common
 
tomato
 
echo

In case of success your 1 BTC will waiting for you at m/84'/0'/0'/0/0 path. We use this mnemonic tool to split code.



tOOLS AND INFO USED TO MAKE THE PUZZLE - 


BIP-00XX : Mnemonic codes improvement
Number:  BIP-00XX
Title:   Mnemonic codes improvement
Type:    Standard
Status:  Draft
Authors: Aleksey Karpov <admin@bitaps.com>
Created: 2020-05-19
Abstract
This BIP describes development recommendations for the generation of deterministic wallets and secure backup strategies.

Motivation
Mnemonic codes proposed in BIP39 are the most successful solution for storing secret keys. A key factor is the use of a human-readable format and the zero chance of information loss due to typos. This BIP is aimed at solving the problem by storing the code by dividing it into parts according to the threshold scheme for sharing the secret, as well as a description of how the mnemonic code can be generated manually without trusting any hardware/software random generators. The scope of application of this BIP is the creation and storage of cryptocurrency wallets backup for personal use.

Manually generation
The mnemonic phrase is inherently a backup of access to the wallet. To ensure reliable backup storage developed metal backups that allow you to save information after exposure to aggressive environments and temperatures. But this approach does not protect mnemonic code against theft or single point trust problem. The threshold secret sharing scheme significantly improves secure backup storage against a single storage point trust problem.

In cryptography, several secret sharing schemes are known. The most famous is Shamir's secret scheme and Blakley's scheme. This BIP is based on Shamir's scheme. With a more detailed description of Shamir's scheme, can be found on If you do not trust any software random number generator due to possible bugs or backdoors, you can generate a mnemonic code using dice yourself. Critical to security is the use of random numbers. Do not try to select words from the mnemonic dictionary in other ways. Even if it seems to you that you are unpredictably doing this, most likely you are mistaken.

In previously known dice generation methods, a large secret number is generated and formatted as a hexadecimal string. Later this hexadecimal string is converted into a mnemonic phrase using a software tool.

This BIP suggests generate a mnemonic phrase using dice in a simpler and more obvious way using the Dice wordlist table.

In the proposed generation scheme, it is most convenient to use 5 cubes at a time. If you have only 1 die, you should make 5 consecutive throws to generate 1 word. With manual dice code generation, it is not possible to calculate the checksum, which is written at the end in accordance with BIP39. Wallets and splitting schema suggested below, requires the correct mnemonic code checksum, the last word MUST be adjusted to correct checksum using a software tool. In case user provided incorrect last word, wallet software should adjust correct checksum on the fly before converting mnemonic to binary seed.

You should select 12 / 15 / 18 / 21 or 24 (recommended) words using dice rolls and dice wordlist table. If your throw result is not on the list, just ignore and continue. Some combinations are excluded for a uniform probability distribution.

Five dices with 6 sides give 6 ^ 5 = 7776 possible combinations. Wordlist is 2048 words, that overlaps three times range for 7776 combinations, with 2048 * 3 = 6144 good values, rest range (6144, 7776] are excluded. Each word in the table corresponds to 3 roll combinations.

BIP39 Dice word list

BIP39 checksum
BIP39 describes an algorithm for converting a secret value (BIP32 seed) into a mnemonic code. This algorithm provides the calculation of the checksum, which is written at the end and affects the last word in the code. This checksum is excessive since there is no practical benefit from it. The number of bits in a checksum is so small that it does not allow you to restore the word order in case it is lost. In the case of a typo, the BIP39 wordlist itself allows you to correct it. Additionally, the presence of a checklist makes it impossible to generate a mnemonic phrase manually for wallets that strictly control checksum. The checksum MUST be adjusted on the fly by Wallets to enable manually generation and protect privacy with the opportunity for plausible deniability.

Splitting mnemonic code
The mnemonic phrase is inherently a backup of access to the wallet. To ensure reliable backup storage developed metal backups that allow you to save information after exposure to aggressive environments and temperatures. But this approach does not protect mnemonic code against theft or single point trust problem. Threshold secret sharing scheme significantly improve secure backup storage against single point trust problem.

In cryptography, several secret sharing schemes are known. The most famous are the Shamir's secret scheme and Blakley's scheme. This BIP is based on the Shamir's scheme. With a more detailed description of Shamir's scheme, scan be found on Wikipedia.

There are already exists two great implementation of the Shamir scheme for mnemonic codes. First one is SLIP39 from Satoshi Labs, and second one is shamir39 from iancoleman.

The main idea of creating this BIP is to create the most simple and understandable scheme for mass use. SLIP39 is a complicated scheme with division into groups and division within groups into shares, a checksum is used to validate shares. This scheme is more likely focused on corporate users. Iancoleman scheme is more simple and more suitable for mass use.

This BIP proposes technical implementation Shamir's secret sharing scheme applied separately to each byte of the shared secret and GF(256) is used as the underlying finite field. Secret stored in share f(0). On GF(256) finite field 255 shares are maximal possible. For the need for personal use, this is more than enough. Share index (x coordinate) should be randomly selected to prevent any information leak about sharing split schema.

Mnemonic code share
The share in appearance and format should not differ from the usual mnemonic code, to protect privacy. To recover the mnemonic code from shares, you need to store and know the share indexes (x coordinate). Since the index is randomly selected and the checksum designed in BIP 39 is redundant, the index can be written into bits reserved for checksum without any leaks about splitting scheme and breaking existed design. Since the number of bits for checksum varies depending on the number of words of the mnemonic code, we have a limit for the maximum number of total shares in the secret sharing scheme, depending on the length of the mnemonic code word.

- 12 words: 4 bits -> 15 total shares
- 15 words: 5 bits -> 31 total shares
- 18 words: 6 bits -> 63 total shares
- 21 words: 7 bits -> 127 total shares
- 24 words: 8 bits -> 255 total shares
The range for random index selection MUST be limited by the maximal number of total shares.

Reference implementation

S.__split_secret = (threshold, total,  secret, indexBits=8) => {
    if (threshold > 255) throw new Error("threshold limit 255");
    if (total > 255) throw new Error("total limit 255");
    let index_mask = 2**indexBits - 1;
    if (total > index_mask) throw new Error("index bits is to low");
    if (threshold > total) throw new Error("invalid threshold");
    let shares = {};
    let sharesIndexes = [];
    let e = S.generateEntropy({hex:false});
    let ePointer = 0;
    let i = 0;
    let index;
    // generate random indexes (x coordinate)
    do {
       if (ePointer >= e.length) {
           // get more 32 bytes entropy
           e = S.generateEntropy({hex:false});
           ePointer = 0;
       }
       index = e[ePointer] & index_mask;
       if ((shares[index] === undefined)&&(index !== 0)) {
           i++;
           shares[index] = BF([]);
           sharesIndexes.push(index)
       }
       ePointer++;
    } while (i !== total);

    e = S.generateEntropy({hex:false});
    ePointer = 0;
    let w;
    for (let b = 0; b < secret.length; b++) {
        let q = [secret[b]];
        for (let i = 0; i < threshold - 1; i++) {
            do {
                if (ePointer >= e.length) {
                    ePointer = 0;
                    e = S.generateEntropy({hex:false});
                }
                w  = e[ePointer++];
            } while (q.includes(w));
            q.push(w);
        }
        for (let i of sharesIndexes)
            shares[i] = BC([shares[i], BF([S.__shamirFn(i, q)])]);

    }
    return shares;
};

    

  


TOOLS - 


module.exports = function (S) {
    let BA = S.Buffer.alloc;
    let BF = S.Buffer.from;
    let BC = S.Buffer.concat;


    S.__precompute_GF256_expLog = (S) => {
        let exp = BA(255, 0);
        let log = BA(256, 0);
        let poly = 1;
        for (let i=0; i < 255; i++) {
            exp[i] = poly;
            log[poly] = i;
            // Multiply poly by the polynomial x + 1.
            poly = (poly << 1) ^ poly;
            // Reduce poly by x^8 + x^4 + x^3 + x + 1
            if (poly & 0x100) poly ^= 0x11b;
        }
        S.GF256_EXP_TABLE = exp;
        S.GF256_LOG_TABLE = log;
    };

    S.__GF256_mul = (a, b) => {
        if ((a === 0) || (b === 0)) return 0;
        return S.GF256_EXP_TABLE[S.__mod(S.GF256_LOG_TABLE[a] + S.GF256_LOG_TABLE[b], 255)];
    };

    S.__GF256_pow = (a, b) => {
        if (b === 0) return 1;
        if (a === 0) return 0;
        let c = a;
        for (let i = 0; i < b-1; i++) c = S.__GF256_mul(c, a);
        return c;
    };

    S.__mod = (a, b) => ((a%b) + b) % b;

    S.__GF256_add = (a, b) => a ^ b;

    S.__GF256_sub = (a, b) => a ^ b;

    S.__GF256_inverse = (a) => {
        if (a === 0) throw new Error("Zero division");
        return S.GF256_EXP_TABLE[S.__mod(-1 * S.GF256_LOG_TABLE[a], 255)];
    };

    S.__GF256_div = (a, b) => {
        if (b === 0) throw new Error("Zero division");
        if (a === 0) return 0;
        let r = S.GF256_EXP_TABLE[S.__mod(S.GF256_LOG_TABLE[a] - S.GF256_LOG_TABLE[b], 255)];
        // let r = S.__GF256_mul(a, S.__GF256_inverse(b));
        if (a !== S.__GF256_mul(r, b)) throw new Error("failed");
        return r;
    };

    S.__shamirFn = (x, q) => {
        let r = 0;
        for (let a of q) r = S.__GF256_add(r, S.__GF256_mul(a, S.__GF256_pow(x, q.indexOf(a))));
        return r;
    };

    S.__shamirInterpolation = (points) => {
        let k = points.length;
        if (k<2) throw new Error("Minimum 2 points required");
        points.sort((a,b) => a[0] - b[0]);
        let z = new Set();
        for (let i of points) z.add(i[0]);
        if (z.size !== points.length) throw new Error("Unique points required");
        let p_x = 0;
        for (let j = 0; j < k; j++) {
            let p_j_x = 1;
            for (let m = 0; m < k; m++) {
                if (m===j) continue;
                // let a = S.__GF256_sub(x, points[m][0]);
                let a = points[m][0];
                // let b = S.__GF256_sub(points[j][0], points[m][0]);
                let b = S.__GF256_add(points[j][0], points[m][0]);
                let c = S.__GF256_div(a, b);
                p_j_x = S.__GF256_mul(p_j_x, c);
            }
            p_j_x = S.__GF256_mul(points[j][1], p_j_x);
            p_x = S.__GF256_add(p_x, p_j_x);
        }
        return p_x;
    };


    S.__split_secret = (threshold, total,  secret, indexBits=8) => {
        if (threshold > 255) throw new Error("threshold limit 255");
        if (total > 255) throw new Error("total limit 255");
        let index_mask = 2**indexBits - 1;
        if (total > index_mask) throw new Error("index bits is to low");
        if (threshold > total) throw new Error("invalid threshold");
        let shares = {};
        let sharesIndexes = [];

        let e = S.generateEntropy({hex:false});
        let ePointer = 0;
        let i = 0;
        let index;

        // generate random indexes (x coordinate)
        do {
           if (ePointer >= e.length) {
               // get more 32 bytes entropy
               e = S.generateEntropy({hex:false});
               ePointer = 0;
           }

           index = e[ePointer] & index_mask;
           if ((shares[index] === undefined)&&(index !== 0)) {
               i++;
               shares[index] = BF([]);
               sharesIndexes.push(index)
           }

           ePointer++;
        } while (i !== total);


        e = S.generateEntropy({hex:false});
        ePointer = 0;

        let w;
        for (let b = 0; b < secret.length; b++) {
            let q = [secret[b]];

            for (let i = 0; i < threshold - 1; i++) {
                do {
                    if (ePointer >= e.length) {
                        ePointer = 0;
                        e = S.generateEntropy({hex:false});
                    }
                    w  = e[ePointer++];
                } while (q.includes(w));
                q.push(w);
            }

            for (let i of sharesIndexes)
                shares[i] = BC([shares[i], BF([S.__shamirFn(i, q)])]);

        }
        return shares;
    };

    S.__restore_secret = (shares) => {
      let secret = BF([]);
      let shareLength = null;
      let q = [];

      for (let i in shares) {
          i = parseInt(i);
          if ((i < 1) || (i > 255)) throw new Error("Invalid share index " + i);
          if (shareLength === null) shareLength = shares[i].length;
          if ((shareLength !== shares[i].length) || (shareLength === 0))  throw new Error("Invalid shares");
      }

      for (let i = 0; i < shareLength; i++) {
          let points = [];
          for (let z in shares) {
              z = parseInt(z);
              points.push([z, shares[z][i]])
          }
          secret = BC([secret, BF([S.__shamirInterpolation(points)])])
      }
      return secret;
    };

    S.__precompute_GF256_expLog(S);
};


(OBJECTIVE COME UP WITH A RED TEAM TOOL TO FIND A BUG TO EXPOSE THE THIRD SHARE TO WIN THE PUZZLE) 
