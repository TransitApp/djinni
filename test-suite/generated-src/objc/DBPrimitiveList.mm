// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from primitive_list.djinni

#import "DBPrimitiveList.h"
#import "DjinniUtils.h"


@implementation DBPrimitiveList

- (nonnull instancetype)initWithList:(nonnull NSArray<NSNumber *> *)list
{
    if (self = [super init]) {
        _list = [list copy];
    }
    return self;
}

+ (nonnull instancetype)primitiveListWithList:(nonnull NSArray<NSNumber *> *)list
{
    return [[self alloc] initWithList:list];
}

- (BOOL)isEqual:(id)other
{
    if (![other isKindOfClass:[DBPrimitiveList class]]) {
        return NO;
    }
    DBPrimitiveList *typedOther = (DBPrimitiveList *)other;
    return [self.list isEqualToArray:typedOther.list];
}

- (NSUInteger)hash
{
    NSUInteger hashCode = 17;
    hashCode = hashCode * 31 + self.list.dynamicHash;
    return hashCode;
}

#ifndef DJINNI_DISABLE_DESCRIPTION_METHODS
- (NSString *)description
{
    return [NSString stringWithFormat:@"<%@ %p list:%@>", self.class, (void *)self, self.list];
}

#endif
@end
