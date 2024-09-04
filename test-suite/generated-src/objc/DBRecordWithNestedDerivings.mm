// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from derivings.djinni

#import "DBRecordWithNestedDerivings.h"
#import "DjinniUtils.h"


@implementation DBRecordWithNestedDerivings

- (nonnull instancetype)initWithKey:(int32_t)key
                                rec:(nonnull DBRecordWithDerivings *)rec
{
    if (self = [super init]) {
        _key = key;
        _rec = rec;
    }
    return self;
}

+ (nonnull instancetype)recordWithNestedDerivingsWithKey:(int32_t)key
                                                     rec:(nonnull DBRecordWithDerivings *)rec
{
    return [[self alloc] initWithKey:key
                                 rec:rec];
}

- (BOOL)isEqual:(id)other
{
    if (![other isKindOfClass:[DBRecordWithNestedDerivings class]]) {
        return NO;
    }
    DBRecordWithNestedDerivings *typedOther = (DBRecordWithNestedDerivings *)other;
    return self.key == typedOther.key &&
            [self.rec isEqual:typedOther.rec];
}

- (NSUInteger)hash
{
    NSUInteger hashCode = 17;
    hashCode = hashCode * 31 + (NSUInteger)self.key;
    hashCode = hashCode * 31 + self.rec.hash;
    return hashCode;
}

- (NSComparisonResult)compare:(DBRecordWithNestedDerivings *)other
{
    NSComparisonResult tempResult;
    if (self.key < other.key) {
        tempResult = NSOrderedAscending;
    } else if (self.key > other.key) {
        tempResult = NSOrderedDescending;
    } else {
        tempResult = NSOrderedSame;
    }
    if (tempResult != NSOrderedSame) {
        return tempResult;
    }
    tempResult = [self.rec compare:other.rec];
    if (tempResult != NSOrderedSame) {
        return tempResult;
    }
    return NSOrderedSame;
}

#ifndef DJINNI_DISABLE_DESCRIPTION_METHODS
- (NSString *)description
{
    return [NSString stringWithFormat:@"<%@ %p key:%@ rec:%@>", self.class, (void *)self, @(self.key), self.rec];
}

#endif
@end
