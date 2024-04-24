//
//  NSArray+Djinni.mm
//  Djinni
//
//  Created by Julien Lepine on 2024-01-08.
//

#import "NSArray+Djinni.h"
#import "UIColor+Djinni.h"

@implementation NSArray (Djinni)

- (NSUInteger)dynamicHash {
    NSUInteger hash = 0;
    for (id obj in self) {
        if ([obj isKindOfClass:[UIColor class]]) {
            hash ^= [obj dynamicHash];
        } else {
            hash ^= [obj hash];
        }
    }
    return hash;
}

@end
