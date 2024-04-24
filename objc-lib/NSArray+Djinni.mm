//
//  NSArray+Djinni.mm
//  Djinni
//
//  Created by Julien Lepine on 2024-01-08.
//

#import "NSArray+Djinni.h"

@implementation NSArray (Djinni)

/**
 * Based on: https://www.mikeash.com/pyblog/friday-qa-2010-06-18-implementing-equality-and-hashing.html
 */
#define NSUINT_BIT (CHAR_BIT * sizeof(NSUInteger))
#define NSUINTROTATE(val, howmuch) ((((NSUInteger)val) << howmuch) | (((NSUInteger)val) >> (NSUINT_BIT - howmuch)))
- (NSUInteger)dynamicHash {
    NSUInteger combinedHash = 0;
    NSUInteger count = self.count;
    for (NSUInteger i = 0; i < count; i++) {
        id obj = self[i];
        NSUInteger objHash = [obj respondsToSelector:@selector(dynamicHash)] ? [obj dynamicHash] : [obj hash];
        NSUInteger rotationAmount = NSUINT_BIT / (i + 1);
        combinedHash ^= NSUINTROTATE(objHash, rotationAmount);
    }
    return combinedHash;
}
@end

