//
//  NSArray+Djinni.mm
//  Djinni
//
//  Created by Julien Lepine on 2024-01-08.
//

#import "NSArray+Djinni.h"

@implementation NSArray (Djinni)

- (NSUInteger)dynamicHash {
    NSUInteger result = 0;
    for (id obj in self) {
        NSUInteger objHash = [obj respondsToSelector:@selector(dynamicHash)] ?
                            [obj dynamicHash] : [obj hash];
        // Boost hash_combine pattern: distributes bits evenly to prevent collisions
        result ^= objHash + 0x9e3779b9 + (result << 6) + (result >> 2);
    }
    return result;
}

@end

