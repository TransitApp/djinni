// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from enum.djinni

#import "DBEnumUsageRecord.h"
#import "DjinniUtils.h"


@implementation DBEnumUsageRecord

- (nonnull instancetype)initWithE:(DBColor)e
                                o:(nullable NSNumber *)o
                                l:(nonnull NSArray<NSNumber *> *)l
                                s:(nonnull NSSet<NSNumber *> *)s
                                m:(nonnull NSDictionary<NSNumber *, NSNumber *> *)m
{
    if (self = [super init]) {
        _e = e;
        _o = o;
        _l = [l copy];
        _s = [s copy];
        _m = [m copy];
    }
    return self;
}

+ (nonnull instancetype)enumUsageRecordWithE:(DBColor)e
                                           o:(nullable NSNumber *)o
                                           l:(nonnull NSArray<NSNumber *> *)l
                                           s:(nonnull NSSet<NSNumber *> *)s
                                           m:(nonnull NSDictionary<NSNumber *, NSNumber *> *)m
{
    return [[self alloc] initWithE:e
                                 o:o
                                 l:l
                                 s:s
                                 m:m];
}

#ifndef DJINNI_DISABLE_DESCRIPTION_METHODS
- (NSString *)description
{
    return [NSString stringWithFormat:@"<%@ %p e:%@ o:%@ l:%@ s:%@ m:%@>", self.class, (void *)self, @(self.e), self.o, self.l, self.s, self.m];
}

#endif
@end
