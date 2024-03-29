# Cosine Similarity

Cosine similarity is a metric used to measure the similarity between two vectors in a high-dimensional space. It determines the cosine of the angle between the two vectors and produces a value ranging from -1 to 1.

To understand cosine similarity, let's consider two vectors, A and B, with n-dimensional components. Each component of the vectors represents a feature or attribute of an object. Cosine similarity measures how similar the directions of the two vectors are, regardless of their magnitudes.

The formula for cosine similarity is:

cosine similarity = (A dot product B) / (||A|| * ||B||)

Here's a breakdown of the formula:

1. Dot product (A dot product B): It is the sum of the products of corresponding components of the two vectors. It measures how much the vectors align in the same direction. Mathematically, it is calculated as:

   A dot product B = A1 * B1 + A2 * B2 + ... + An * Bn

2. Vector magnitudes (||A|| and ||B||): They represent the lengths or magnitudes of the vectors A and B, respectively. The magnitude of a vector is calculated as the square root of the sum of the squares of its components. Mathematically, it is calculated as:

   ||A|| = sqrt(A1^2 + A2^2 + ... + An^2)
   ||B|| = sqrt(B1^2 + B2^2 + ... + Bn^2)

3. Cosine similarity: It is obtained by dividing the dot product of A and B by the product of their magnitudes. The resulting value ranges from -1 to 1. If the vectors are exactly the same, the cosine similarity is 1. If they are orthogonal (perpendicular), the cosine similarity is 0. If they are in opposite directions, the cosine similarity is -1.

   cosine similarity = (A dot product B) / (||A|| * ||B||)

Cosine similarity is commonly used in various fields, including natural language processing, information retrieval, and recommendation systems. In text analysis, for example, documents are often represented as high-dimensional vectors, and cosine similarity is used to determine how similar two documents are based on their word frequencies or embeddings.

**Generated by ChatGPT**

## References
* https://blogs.sas.com/content/iml/2019/09/03/cosine-similarity.html