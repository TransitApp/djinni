// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#import "SPLevelDViewModel.h"
#import "DjinniUtils.h"


@implementation SPLevelDViewModel

- (nonnull instancetype)initWithFieldA:(nonnull NSString *)fieldA
                                fieldB:(nonnull NSString *)fieldB
                                fieldC:(nonnull NSString *)fieldC
                                fieldD:(nonnull NSString *)fieldD
{
    if (self = [super initWithFieldA:fieldA
                              fieldB:fieldB
                              fieldC:fieldC])
    {
        _fieldD = [fieldD copy];
    }
    return self;
}

+ (nonnull instancetype)LevelDWithFieldA:(nonnull NSString *)fieldA
                                  fieldB:(nonnull NSString *)fieldB
                                  fieldC:(nonnull NSString *)fieldC
                                  fieldD:(nonnull NSString *)fieldD
{
    return [[self alloc] initWithFieldA:fieldA
                                 fieldB:fieldB
                                 fieldC:fieldC
                                 fieldD:fieldD];
}

#ifndef DJINNI_DISABLE_DESCRIPTION_METHODS
- (NSString *)description
{
    return [NSString stringWithFormat:@"<%@ %p fieldA:%@ fieldB:%@ fieldC:%@ fieldD:%@>", self.class, (void *)self, self.fieldA, self.fieldB, self.fieldC, self.fieldD];
}

#endif
@end
