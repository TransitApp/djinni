//
//  NSArray+Djinni.mm
//  Djinni
//
//  Created by Julien Lepine on 2024-01-08.
//

#import "NSArray+Djinni.h"
#import "UIColor+Djinni.h"

@implementation NSArray (Djinni)

/**
 * Based on: https://www.mikeash.com/pyblog/friday-qa-2010-06-18-implementing-equality-and-hashing.html
 */
#define NSUINT_BIT (CHAR_BIT * sizeof(NSUInteger))
#define NSUINTROTATE(val, howmuch) ((((NSUInteger)val) << howmuch) | (((NSUInteger)val) >> (NSUINT_BIT - howmuch)))

- (NSUInteger)dynamicHash {
    NSUInteger hash = 0;
    for (NSUInteger i = 0; i < self.count; i++) {
        id obj = self[i];
        NSUInteger objHash = [obj isKindOfClass:[UIColor class]] ? [obj dynamicHash] : [obj hash];
        hash ^= (i % 2 == 0) ? objHash : NSUINTROTATE(objHash, NSUINT_BIT / 2);
    }
    return hash;
}

@end

