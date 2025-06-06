// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from proto.djinni

#import "DBRecordWithEmbeddedCppProto.h"
#import "DjinniUtils.h"


@implementation DBRecordWithEmbeddedCppProto

- (nonnull instancetype)initWithState:(const djinni::test2::PersistingState & )state
{
    if (self = [super init]) {
        _state = state;
    }
    return self;
}

+ (nonnull instancetype)RecordWithEmbeddedCppProtoWithState:(const djinni::test2::PersistingState & )state
{
    return [[self alloc] initWithState:state];
}

- (BOOL)isEqual:(id)other
{
    if (![other isKindOfClass:[DBRecordWithEmbeddedCppProto class]]) {
        return NO;
    }
    DBRecordWithEmbeddedCppProto *typedOther = (DBRecordWithEmbeddedCppProto *)other;
    return ;
}

- (NSUInteger)hash
{
    NSUInteger hashCode = 17;
    hashCode = hashCode * 31 + ();
    return hashCode;
}

#ifndef DJINNI_DISABLE_DESCRIPTION_METHODS
- (NSString *)description
{
    return [NSString stringWithFormat:@"<%@ %p state:%@>", self.class, (void *)self, @"[Opaque C++ Protobuf object]"];
}

#endif
@end
