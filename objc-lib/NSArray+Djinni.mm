//
//  NSArray+Djinni.mm
//  Djinni
//
//  Created by Julien Lepine on 2024-01-08.
//

#import "NSArray+Djinni.h"

@implementation NSArray (Djinni)

- (NSUInteger)dynamicHash {
    NSUInteger result = 1;
    NSUInteger prime = 31;
    for (id obj in self) {
        NSUInteger objHash = [obj respondsToSelector:@selector(dynamicHash)] ? [obj dynamicHash] : [obj hash];
        result = prime * result + objHash;
    }
    return result;
}

@end

