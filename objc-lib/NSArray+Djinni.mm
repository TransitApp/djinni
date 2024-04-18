//
//  NSArray+Djinni.mm
//  Djinni
//
//  Created by Julien Lepine on 2024-01-08.
//

#import "NSArray+Djinni.h"

@implementation NSArray (Djinni)

- (NSUInteger)dynamicHash {
    NSUInteger hash = 0;
    for (id obj in self) {
        auto objHash = [obj hash];
        hash ^= objHash;
    }
    return hash;
}

@end
