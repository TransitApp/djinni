// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from date.djinni

#import "DBMapDateRecord.h"
#import "DjinniUtils.h"


@implementation DBMapDateRecord

- (nonnull instancetype)initWithDatesById:(nonnull NSDictionary<NSString *, NSDate *> *)datesById
{
    if (self = [super init]) {
        _datesById = [datesById copy];
    }
    return self;
}

+ (nonnull instancetype)mapDateRecordWithDatesById:(nonnull NSDictionary<NSString *, NSDate *> *)datesById
{
    return [[self alloc] initWithDatesById:datesById];
}

#ifndef DJINNI_DISABLE_DESCRIPTION_METHODS
- (NSString *)description
{
    return [NSString stringWithFormat:@"<%@ %p datesById:%@>", self.class, (void *)self, self.datesById];
}

#endif
@end
