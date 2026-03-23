package com.longbridge.content;

import java.util.concurrent.CompletableFuture;

import com.longbridge.*;

/**
 * Content context
 */
public class ContentContext implements AutoCloseable {
    private long raw;

    /**
     * Create a ContentContext object
     *
     * @param config Config object
     * @return A ContentContext object
     */
    public static ContentContext create(Config config) {
        ContentContext ctx = new ContentContext();
        ctx.raw = SdkNative.newContentContext(config.getRaw());
        return ctx;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeContentContext(raw);
    }

    /**
     * Get discussion topics list
     *
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<TopicItem[]> getTopics(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.contentContextTopics(raw, symbol, callback);
        });
    }

    /**
     * Get news list
     *
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<NewsItem[]> getNews(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.contentContextNews(raw, symbol, callback);
        });
    }
}
