// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#import "SPLevelB2ViewModel.h"


@implementation SPLevelB2ViewModel

- (nonnull instancetype)initWithFieldA:(nonnull NSString *)fieldA
                                fieldB:(nonnull NSString *)fieldB
                               fieldB2:(nonnull NSString *)fieldB2
{
    if (self = [super initWithFieldA:fieldA
                              fieldB:fieldB])
    {
        _fieldB2 = [fieldB2 copy];
    }
    return self;
}

+ (nonnull instancetype)LevelB2WithFieldA:(nonnull NSString *)fieldA
                                   fieldB:(nonnull NSString *)fieldB
                                  fieldB2:(nonnull NSString *)fieldB2
{
    return [[self alloc] initWithFieldA:fieldA
                                 fieldB:fieldB
                                fieldB2:fieldB2];
}

#ifndef DJINNI_DISABLE_DESCRIPTION_METHODS
- (NSString *)description
{
    return [NSString stringWithFormat:@"<%@ %p fieldA:%@ fieldB:%@ fieldB2:%@>", self.class, (void *)self, self.fieldA, self.fieldB, self.fieldB2];
}

#endif
@end