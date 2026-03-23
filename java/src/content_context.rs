use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{Config, content::ContentContext};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, ObjectArray},
};

struct ContextObj {
    ctx: ContentContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newContentContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let ctx = ContentContext::new(config);
        Ok(Box::into_raw(Box::new(ContextObj { ctx })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeContentContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_contentContextTopics(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.topics(symbol).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_contentContextNews(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.news(symbol).await?))
        })?;
        Ok(())
    })
}
