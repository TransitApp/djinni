// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#import "SPLevelEViewModel.h"
#import "DjinniUtils.h"


@implementation SPLevelEViewModel

- (nonnull instancetype)initWithFieldA:(nonnull NSString *)fieldA
                                fieldB:(nonnull NSString *)fieldB
                                fieldC:(nonnull NSString *)fieldC
                                fieldD:(nonnull NSString *)fieldD
                                fieldE:(nonnull NSString *)fieldE
{
    if (self = [super initWithFieldA:fieldA
                              fieldB:fieldB
                              fieldC:fieldC
                              fieldD:fieldD])
    {
        _fieldE = [fieldE copy];
    }
    return self;
}

+ (nonnull instancetype)LevelEWithFieldA:(nonnull NSString *)fieldA
                                  fieldB:(nonnull NSString *)fieldB
                                  fieldC:(nonnull NSString *)fieldC
                                  fieldD:(nonnull NSString *)fieldD
                                  fieldE:(nonnull NSString *)fieldE
{
    return [[self alloc] initWithFieldA:fieldA
                                 fieldB:fieldB
                                 fieldC:fieldC
                                 fieldD:fieldD
                                 fieldE:fieldE];
}

#ifndef DJINNI_DISABLE_DESCRIPTION_METHODS
- (NSString *)description
{
    return [NSString stringWithFormat:@"<%@ %p fieldA:%@ fieldB:%@ fieldC:%@ fieldD:%@ fieldE:%@>", self.class, (void *)self, self.fieldA, self.fieldB, self.fieldC, self.fieldD, self.fieldE];
}

#endif
@end
