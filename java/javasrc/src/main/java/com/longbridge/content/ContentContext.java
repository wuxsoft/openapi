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
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public static CompletableFuture<ContentContext> create(Config config)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.newContentContext(config.getRaw(), callback);
        });
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeContentContext(raw);
    }

    /**
     * Get news list
     *
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.content.*;
     *
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *                 .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth);
     *                 ContentContext ctx = ContentContext.create(config).get()) {
     *             NewsItem[] items = ctx.getNews("700.HK").get();
     *             for (NewsItem item : items) {
     *                 System.out.println(item);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     *
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
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

    public CompletableFuture<NewsItem[]> getNews(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.contentContextNews(raw, symbol, callback);
        });
    }
}
